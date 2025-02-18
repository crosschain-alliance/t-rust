import io
import tarfile
import docker
import getpass
import grp
import os
import signal
import subprocess
import sys


def is_docker_installed():
    """
    Check if docker is installed on the host

    Returns:
        True if installed, False if not
    """
    result = subprocess.run(['which', 'docker'], capture_output=True, text=True)
    if result.returncode == 0:
        return True
    else:
        print('[!] please install docker first')
        return False

def is_user_in_docker_group():
    """
    Check if $USER is in docker group

    Returns:
        True if in group, False if not
    """
    user = getpass.getuser()
    groups = [g.gr_name for g in grp.getgrall() if user in g.gr_mem]
    if 'docker' in groups:
        return True
    else:
        print(f'[!] please add user {user} in the docker group')
        return False

def can_run_docker_ps():
    """
    Check if $USER can run `docker ps`

    Returns:
        True if able, False if not
    """
    try:
        result = subprocess.run(['docker', 'ps'], capture_output=True, text=True)
        if result.returncode == 0:
            return True
        else:
            print(f'[!] please restart the docker service or open a new console')
            return False
    except FileNotFoundError:
        print(f'[!] please restart the docker service or open a new console')
        return False

# docker checks
if is_docker_installed():
    if is_user_in_docker_group():
        if can_run_docker_ps():
            client = docker.from_env()
        else:
            sys.exit(1)
    else:
        sys.exit(1)
else:
    sys.exit(1)

def stop_container(cont_id, verbose=False):
    """
    Stop container by id

    Args:
        cont_id (str): container id
    """
    try:
        if verbose:
            print(f'Waiting for t-rust process to end ...')
        container = client.containers.get(cont_id)
        container.stop()
        container.wait()
        if verbose:
            print(f'Done')
    except docker.errors.NotFound:
        pass
    except docker.errors.APIError as e:
        print(f'[!] error while stopping container: {e}')

def remove_container(cont_name):
    """
    Delete container by name

    Args:
        cont_name (str): container name
    """
    try:
        container = client.containers.get(cont_name)
        if check_if_container_is_running(cont_name):
            stop_container(container.id)
        container.remove()
    except docker.errors.NotFound:
        pass
    except docker.errors.APIError as e:
        print(f'[!] error while deleting container: {e}')

def cleanup(label):
    """
    Cleanup containers and images (by label/name and dangling)

    Args:
        label (str): container name (== to container label)
    """
    try:
        client.containers.prune()
        try:
            image = client.images.get(f'{label}:latest')
            client.images.remove(image.id, force=True)
        except docker.errors.APIError:
            pass
        client.images.prune()
        client.images.prune(filters={'dangling': True})
        for image in client.images.list(filters={"reference": "<none>:<none>"}):
            try:
                client.images.remove(image.id, force=True)
            except docker.errors.APIError as e:
                print(f'[!] error while cleaning up containers/images: {e}')
    except docker.errors.APIError as e:
        print(f'[!] error while cleaning up containers/images: {e}')

def create_signal_handler(label):
    """
    Closure to create signal

    Args:
        label (str): container name (== to container label)
    """
    def signal_handler(signum, frame):
        """
        Manage signals

        Args:
            signum (int): signal code
            frame (frame): execution context info

        Returns:
            signal handler
        """
        if signum:
            containers = client.containers.list(all=True, filters={'label': label})
            for container in containers:
                stop_container(container.id)
            cleanup(label)
            sys.exit(0)
    return signal_handler

def check_if_container_is_running(cont_name):
    """
    Check if the given container name is in list of running containers

    Args:
        cont_name (str): container name

    Returns:
        True if it's running, False if not
    """
    containers_list = client.containers.list()
    containers_names_list = [cont.name for cont in containers_list]
    if cont_name in containers_names_list:
        return True
    return False

def run_existing_container(action, cont_name, target, mode_value, verbose, file_path=None):
    """
    Run an existing container given its action and mode

    Args:
        action (str): action to run
        cont_name (str): container name
        target (str): backend type
        mode_value (str): run mode
        verbose (bool): verbose flag
        file_path (str, optional): file to pass in the container. Defaults to None.
    """
    if action == 'compile':
        print(f'Compiling for {target} target ...')
    elif action == 'run':
        print(f'Running {target} target ...')
    container = client.containers.get(cont_name)    
    if container.status != 'running':
        container.start()

    target_path = f'/{target}_target'
    if file_path:
        tar_stream = io.BytesIO()
        with tarfile.open(fileobj=tar_stream, mode='w:gz') as tar:
            tar.add(file_path, arcname='input.file')
        tar_stream.seek(0)
        container.put_archive(target_path, tar_stream)

    container_r = container.exec_run(
                detach=False, stdout=True, stderr=True, stream=True, tty=True,
                environment={'ACTION': action,
                            'TRUST_DOCKER_ABS_PATH': target_path},
                cmd=[f'{target_path}/run.sh', mode_value]
            )
            
    for output in container_r.output:
        if output:
            print(output.decode('utf-8', errors='replace'), end='', flush=True)
    print()

    stop_container(container.id, verbose)

def run_container(action, target, project_path, mode_value, verbose, file_path=None):
    """
    Run a container given action, target and project_path and mode

    Args:
        action (str): action to run
        target (str): backend type
        project_path (str): path of the project
        mode_value (str): run mode
        verbose (bool): verbose flag
        file_path (str, optional): file to pass in the container. Defaults to None.
    """
    cont_name = f'trust_{target}_1'
    signal_handler_instance = create_signal_handler(cont_name)
    signal.signal(signal.SIGINT, signal_handler_instance)  # ctrl-c
    signal.signal(signal.SIGTERM, signal_handler_instance)  # kill
    signal.signal(signal.SIGHUP, signal_handler_instance)  # death of controlling process
    signal.signal(signal.SIGQUIT, signal_handler_instance)  # ctrl-\
    image = f'{cont_name}:latest'
    target_path = f'/{target}_target'
    try:
        if not check_if_container_is_running(cont_name):
            abs_path = os.getenv('TRUST_ABS_PATH')
            if not abs_path:
                print(f'[!] error while reading global var `TRUST_ABS_PATH`')
                sys.exit(1)
            volumes = {
                f'{abs_path}/module/backends/{target}/docker/run.sh':
                    {'bind': f'{target_path}/run.sh', 'mode': 'ro'},
                f'{abs_path}/module/backends/{target}/res/':
                    {'bind': f'{target_path}/res/', 'mode': 'ro'},
                f'{abs_path}/t-rust/':
                    {'bind': f'{target_path}/t-rust/', 'mode': 'ro'},
                f'{abs_path}/args/':
                    {'bind': f'{target_path}/args/', 'mode': 'ro'},
                project_path:
                    {'bind': f'{target_path}/code/', 'mode': 'ro'},
                '/tmp/proofs/':
                    {'bind': '/tmp/proofs/'},
                '/tmp/trust.rargs':
                    {'bind': '/tmp/trust.rargs'}
            }
            try:
                if action == 'compile':
                    remove_container(cont_name)

                existing_images = client.images.list()
                image_exists = any(image in img.tags for img in existing_images)
                if not image_exists:
                    print(f'{target} environment not found')
                    print(f'Building {target} environment ...')
                    if not verbose:
                        print(f'Use --verbose for more details')
                    build = client.api.build(path=f'{abs_path}/module/backends/{target}/docker/',
                                            tag=cont_name, rm=True, decode=True)
                    for line in build:
                        if verbose:
                            print('verbose')
                            if 'stream' in line:
                                print(line['stream'].strip())
                            print(f'\n[+] `{image}` built successfully\n')

                containers = client.containers.list(all=True, filters={'name': cont_name})
                if containers:
                    run_existing_container(action, cont_name, target, mode_value, verbose, file_path)
                else:
                    container = client.containers.run(
                        image, detach=True, remove=False,
                        command="bash",
                        name=cont_name, volumes=volumes,
                        stdin_open=True, tty=True,
                        stdout=True, stderr=True, labels={cont_name: ''}
                    )
                    run_existing_container(action, cont_name, target, mode_value, verbose, file_path)
            except docker.errors.ImageNotFound as e:
                print(f'Failed to build {target} environment image: \n{e}')
            except docker.errors.APIError as e:
                print(f'API error for {target} environment container: \n{e}')
            except Exception as e:
                print(f'[!] error while running container: {e}')
        else:
            print('f')
            container = client.containers.get(cont_name)
            stop_container(container.id)
            run_container(action, target, project_path, mode_value, verbose, file_path)
    except Exception as e:
        print(f'[!] error while running container {cont_name}: {e}')
        sys.exit(1)
    finally:
        signal_handler_instance(None, None)
