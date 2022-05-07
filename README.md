# cogito

This is a project to help learn data science and engineering as well as the rust ecosystem.  It will
also use some scala3 and python3.10.  As a cargo workspace project, it will contain several subprojects:

- alonzo: FP library in rust
- cantor: rust macro library to create data subsets
- cluster-fudge: Project to help setup a k8s raspberry pi4 cluster
    - airflow: Dockerfiles to set up airflow on the cluster
    - spark: playbooks to set up Spark
    - microk8s: playbooks to setup microk8s on raspberry pi4
    - ballista: Dockerfiles and k8s for ballista
- crossbow: DataFusion+Ballista project for test result analytics
- githooks: scripts for githooks
- ignite: port of crossbow for spark 
- lambda-example: rust aws lambda example
- notebooks: set of notebooks on various topics
- reveles: create from scratch neural networks to learn machine learning
- serverless-fe203: serverless framework plugin to build rust lambda
- utils: utilities for various projects

### Usage

This is a normal cargo workspace project. You can compile code with `cargo build`.  Some of the subprojects have Dockerfiles 

For more information on the sbt-dotty plugin, see the

## Development


