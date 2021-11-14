# cluster-fudge

A set of ansible playbooks to help quickly setup various clusters

- micro k8s: Sets up a 5 node raspberry pi4 system to set up a base kubernetes cluster
- zap: Creates a spark cluster using kubernetes
- venturi: Creates an airflow cluster using kubernetes
- minnie: Creates s3 io compatible storage
- ballista: Creates a distributed datafusion cluster for kubernetes

It also has a script to help create default role layouts, based on [ansible best practices][-layout]


- [layout]: https://docs.ansible.com/ansible/2.8/user_guide/playbooks_best_practices.html#directory-layout