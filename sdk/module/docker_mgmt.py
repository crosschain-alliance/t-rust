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

def stop_container(cont_id):
    """
    Stop container by id

    Args:
        cont_id (str): container id
    """
    try:
        container = client.containers.get(cont_id)
        container.stop()
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

def run_existing_container(action, cont_name, target, mode_value):
    """
    Run an existing container given its action and mode

    Args:
        action (str): action to run
        cont_name (str): container name
        target (str): backend type
        mode_value (str): run mode
    """
    container = client.containers.get(cont_name)
    container.start()
    container_r = container.exec_run(detach=False, stdout=True, stderr=True,
                                     environment={'ACTION': action},
                                     cmd=[f'{target}/run.sh', mode_value])
    if container_r.output:
        print(container_r.output.decode('utf-8', errors='replace'))

def run_container(action, target, project_path, mode_value, verbose):
    """
    Run a container given action, target and project_path and mode

    Args:
        action (str): action to run
        target (str): backend type
        project_path (str): path of the project
        mode_value (str): run mode
        verbose (bool): verbose flag
    """
    cont_name = f'trust_{target}_1'
    signal_handler = create_signal_handler(cont_name)
    signal.signal(signal.SIGINT, signal_handler)  # ctrl-c
    signal.signal(signal.SIGTERM, signal_handler)  # kill
    signal.signal(signal.SIGHUP, signal_handler)  # death of controlling process
    signal.signal(signal.SIGQUIT, signal_handler)  # ctrl-\
    image = f'{cont_name}:latest'
    target_path = f'/{target}_target'
    try:
        if not check_if_container_is_running(cont_name):
            abs_path = os.getenv('TRUST_ABS_PATH')
            if not abs_path:
                print(f'[!] error while reading global var `TRUST_ABS_PATH`')
                sys.exit(1)
            volumes = {
                f'{abs_path}/module/backends/{target}/res/':
                    {'bind': f'{target_path}/res/'},
                f'{abs_path}/t-rust/':
                    {'bind': f'{target_path}/t-rust/'},
                f'{abs_path}/args/':
                    {'bind': f'{target_path}/args/'},
                project_path:
                    {'bind': f'{target_path}/code/'},
                '/tmp/trust.rargs/':
                    {'bind': '/tmp/trust.rargs/'}
                }
            try:
                if action == 'compile':
                    remove_container(cont_name)
                container = client.containers.run(image, detach=False, remove=False,
                                                  environment={'ACTION': action},
                                                  command=[f'{target_path}/run.sh', mode_value],
                                                  name=cont_name, volumes=volumes,
                                                  stdout=True, stderr=True, labels={cont_name: ''})
                print(container.decode('utf-8', errors='replace'))
            except docker.errors.ImageNotFound:
                build = client.api.build(path=f'{abs_path}/module/backends/{target}/docker/',
                                         tag=cont_name, rm=True, decode=True)
                if verbose:
                    for line in build:
                        if 'stream' in line:
                            print(line['stream'].strip())
                    print(f'\n[+] `{image}` built successfully\n')
                run_container(action, target, project_path, mode_value, verbose)
            except docker.errors.APIError:
                run_existing_container(action, cont_name, target_path, mode_value)
            except Exception as e:
                print(f'[!] error while running container: {e}')
        else:
            pass
    except docker.errors.APIError:
        run_existing_container(action, cont_name, target_path, mode_value)
    except Exception as e:
        print(f'[!] error while running container {cont_name}: {e}')
        sys.exit(1)
    finally:
        signal_handler(None, None)
