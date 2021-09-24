# cogito

This is a project to help learn data science and engineering as well as the scala3 ecosystem.  It will
also use some rust and python3.10.  As a multiple sbt project, it will contain several subprojects:

- cluster-fudge: Project to help setup a k8s raspberry pi4 cluster
    - venturi: playbooks to set up airflow on the cluster
    - zap: playbooks to set up Spark
    - minnie: playbooks to setup minio s3 compatible storage
    - franz: playbooks to setup kafka cluster
- ignite: a project to help learn spark by writing test results as parquet and doing analytics
- reveles: create from scratch neural networks to learn machine learning
- notebooks: set of notebooks on various topics
- fuse: port of ignite to use DataFusion instead of Spark
- alonzo: FP library in scala3
- sentinel: an http4s or warp server listening to SQS and kafka
- venturi: airflow service with dag examples
- crucible: examines test results, logs, code, and metrics to help find patterns and anomalies
- jupes: scala3 jupyter kernel

### Usage

This is a normal sbt project. You can compile code with `sbt compile`, run it with `sbt run`, and `sbt console` will start a Scala 3 REPL.

For more information on the sbt-dotty plugin, see the
[scala3-example-project](https://github.com/scala/scala3-example-project/blob/main/README.md).
