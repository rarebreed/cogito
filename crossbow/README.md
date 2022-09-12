# crossbow

Crossbow is a project to help you manage data in your test results.  It's time for test data to join the Big Data world

## Motivation

Typically, test results are stored in some kind of proprietary database and ingested from results as they come in from
some sort of test runner.  Quite often, the format is some kind of xunit report that is stored in Jenkins (or some other
CI system) which is also often uploaded to the proprietary test case database system.

Usually, this test case db system is the backend storage used by some kind of test case reporting. The reports are used
by managers who want to use the test results to get a snapshot or trend of the health of a feature.  I believe this
leads to a couple of problems.

- Most test case databases are still SQL 
    - Not designed for huge data sets
    - Not designed for secondary data like logs or service metrics
- While xunit report style is very common, there should be a common schema for what is needed
- It doesn't hook well into other APM dashboards (Kibana, New Relic, Sumo, Data Dog, etc)
    - A dashboard should be a one stop shop

The idea behind crossbow therefore, is to allow massive test result data sets, along with secondary metatdata, like
system metrics, logs or other relevant data, that has a well defined schema, and can be put into a dashboard system
easily.

## Why rust

Currently, JVM tools like Spark and Flink are the most popular for big data crunching.  However, Big Data requires lots
of memory and processing power, both of which are not well served by the JVM. Libraries like arrow2 and frameworks like
datafusion are bringing rust to the data engineering world.

Many benchmarks have shown that rust programs can achieve 200x improvements on memory use, and 2-3x the performance.
When compute and storage costs can run in the multiple millions per year, it more than justifies using rust, even if
it's not as "popular" as JVM languages.  While most people say that python or java are "fast enough", they clearly have
not worked in the realm where speed and cost really do matter, and savings for a company could run up into the millions
per year).
