# cogito

This is a project to help learn data science and engineering as well as the rust ecosystem.  It will
also use some scala3 and python3.10.  As a cargo workspace project, it will contain several subprojects:

- cluster-fudge: Project to help setup a k8s raspberry pi4 cluster
    - venturi: playbooks to set up airflow on the cluster
    - zap: playbooks to set up Spark
    - minnie: playbooks to setup minio s3 compatible storage
    - franz: playbooks to setup kafka cluster
- ignite: a project to help learn spark by writing test results as parquet and doing analytics
- reveles: create from scratch neural networks to learn machine learning
- notebooks: set of notebooks on various topics
- crossbow: port of ignite to use DataFusion+Ballista instead of Spark
- alonzo: FP library in rust
- sentinel: an actix-rs server listening to SQS and kafka
- venturi: airflow service with dag examples
- crucible: examines test results, logs, code, and metrics to help find patterns and anomalies

### Usage

This is a normal cargo workspace project. You can compile code with `cargo build`.  Some of the subprojects have Dockerfiles 

For more information on the sbt-dotty plugin, see the
