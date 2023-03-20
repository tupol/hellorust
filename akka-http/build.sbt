name := "akka-http"

version := "1.0"

scalaVersion := "2.13.1"

lazy val akkaVersion       = "2.6.20"
lazy val akkaHttpVersion   = "10.2.10"
lazy val postgresVersion   = "42.5.4"
lazy val hikariVersion     = "5.0.1"
lazy val logbackVersion    = "1.2.3"
lazy val akkaJson4sVersion = "1.39.2"
lazy val json4sVersion     = "4.0.6"
lazy val commCodecVersion  = "1.15"
lazy val jwtVersion        = "9.2.0"
lazy val bCastleVersion    = "1.70"

libraryDependencies ++= Seq(
  "com.typesafe.akka"    %% "akka-stream"       % akkaVersion,
  "com.typesafe.akka"    %% "akka-http"         % akkaHttpVersion,
  "commons-codec"        % "commons-codec"      % commCodecVersion,
  "ch.qos.logback"       % "logback-classic"    % logbackVersion,
  "org.postgresql"       % "postgresql"         % postgresVersion,
  "com.zaxxer"           % "HikariCP"           % hikariVersion,
  "de.heikoseeberger"    %% "akka-http-json4s"  % akkaJson4sVersion,
  "org.json4s"           %% "json4s-native"     % json4sVersion,
  "org.json4s"           %% "json4s-ext"        % json4sVersion,
  "com.github.jwt-scala" %% "jwt-core"          % jwtVersion,
  "com.github.jwt-scala" %% "jwt-json4s-native" % jwtVersion,
  "org.bouncycastle"     % "bcpkix-jdk15on"     % bCastleVersion,

  "com.typesafe.akka" %% "akka-actor-testkit-typed" % akkaVersion % Test,
  "org.scalatest"     %% "scalatest" % "3.1.0" % Test
)
