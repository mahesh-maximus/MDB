# Getting Started with MDB

## Setting Up Docker

To get Docker, you can either use the
[official Docker install instructions](https://docs.docker.com/install/)
, or the package manager available on your specific Linux distribution:

- on Debian / Ubuntu

  ```bash
  sudo apt-get update
  sudo apt-get install docker.io
  ```

- on Fedora / CentOS / RHEL / Amazon Linux

  ```bash
  sudo yum install docker
  ```

Then, for any of the above, you will need to start the Docker daemon
and add your user to the `docker` group.

```bash
sudo systemctl start docker
sudo usermod -aG docker $USER
```

Don't forget to log out and then back in again, so that the user
change takes effect.

If you wish to have Docker started automatically after boot, you can:

```bash
sudo systemctl enable docker
```

We recommend testing your Docker configuration by running a lightweight
test container and checking for net connectivity:

```bash
docker pull alpine
docker run --rm -it alpine ping -c 3 amazon.com
```