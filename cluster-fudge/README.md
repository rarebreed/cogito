# cluster-fudge

A set of ansible playbooks to help quickly setup various clusters

- micro k8s: Sets up a 5 node raspberry pi4 system to set up a base kubernetes cluster
- zap: Creates a spark cluster using kubernetes
- venturi: Creates an airflow cluster using kubernetes
- minnie: Creates s3 io compatible storage
- ballista: Creates a distributed datafusion cluster for kubernetes

It also has a script to help create default role layouts, based on [ansible best practices][-layout]

## Prerequisistes to running

Before running any of the playbooks, there's some setup that needs to be done (this is why I don't understand why people
think Ansible is better than Salt...there's still a lot of setup that needs to be done)

On laptop

- Create a virtual env (eg `pyenv virtualenv 3.10.2 name`)
- Upgrade ansible: `pip install -U pip`
- Install ansible: `pip install ansible`

For raspberry pi4s

- Install Ubuntu on SD cards (aarch64 server version)
- For one of the raspberry pi 4's, chose it to be main
- Install Ubuntu on all the rpi4's
    - remember the password
- Setup static IP on router for the MAC address of each rpi4
- Write down the hostnames into inventory
    - Alternatively, come up with script for dynamic inventory
- Run `hostnamectl set-hostname rpi4-worker{num}` for all the workers
- Run `hostnamectl set-hosthname rpi4-main` for the main node

Back on laptop

- Run ssh-copy-id ~/.ssh/id_rsa.pub {rpi4-ip-address} to allow passwordless ssh
- Test that you can ssh in
- Run the ansible playbook: `ansible-playbook -i cluster-fudge/micro_k8s/inventory/cluster.yaml cluster-fudge/micro_k8s/mk8s.yml`


- [layout]: https://docs.ansible.com/ansible/2.8/user_guide/playbooks_best_practices.html#directory-layout
