val scala3Version = "3.0.1"

lazy val commonDeps = Seq(
  libraryDependencies += "com.novocode" % "junit-interface" % "0.11" % "test"
)

lazy val root = project
  .in(file("."))
  .settings(
    name := "scala3-simple",
    version := "0.1.0",

    scalaVersion := scala3Version,

    commonDeps
  )

lazy val reveles = project
  .in(file("reveles"))
  .settings(
    name := "rveles",
    version := "0.1.0",
    scalaVersion := scala3Version

    commonDeps
  )
