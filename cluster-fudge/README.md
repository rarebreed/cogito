# cluster-fudge

A set of ansible playbooks to help quickly setup various clusters

- micro k8s: Sets up a 5 node raspberry pi4 system to set up a base kubernetes cluster
- zap: Creates a spark cluster using kubernetes
- venturi: Creates an airflow cluster using kubernetes
- minnie: Creates s3 io compatible storage
- ballista: Creates a distributed datafusion cluster for kubernetes

It also has a script to help create default role layouts, based on [ansible best practices][-layout]

## Prerequisistes to running

Before running any of the playbooks, there's some setup that needs to be done (this is why I don't understand why people think Ansible is better than Salt...there's still a lot of setup that needs to be done)

- Install Ubuntu on SD cards (aarch64 server version)
- For one of the raspberry pi 4's, chose it to be main
- Setup static IP on router for the MAC address of each rpi4
- Write down the hostnames into inventory
- Run `hostnamectl rpi4-worker{num}` for all the workers
- Run `hostnamectl rpi4-main` for the main node
- Run ssh-copy-id ~/.ssh/id_rsa.pub {rpi4-ip-address} to allow passwordless ssh


- [layout]: https://docs.ansible.com/ansible/2.8/user_guide/playbooks_best_practices.html#directory-layout