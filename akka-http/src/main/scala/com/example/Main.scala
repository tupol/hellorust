package com.example

import akka.actor.ActorSystem
import akka.http.scaladsl.Http
import com.example.auth.TokenCreator
import com.example.db.HikariConnectionPool
import com.example.route.TokenRoute
import com.zaxxer.hikari.HikariDataSource
import org.bouncycastle.jce.provider.BouncyCastleProvider

import scala.io.StdIn

object Main {

  def main(args: Array[String]): Unit = {
    implicit val system           = ActorSystem("akka-http-simple-auth")
    implicit val executionContext = system.dispatcher

    val config = SimpleAuthConfig.load()

    // make sure we have a crypto implementation
    java.security.Security.addProvider(new BouncyCastleProvider())

    val datasource    = HikariConnectionPool(new HikariDataSource(config.db.hikari))
    val tokenCreator  = TokenCreator(config.auth)
    val route         = TokenRoute(datasource, tokenCreator).route
    val bindingFuture = Http().newServerAt(config.http.interface, config.http.port).bind(route)

    StdIn.readLine()
    bindingFuture
      .flatMap(_.unbind())
      .onComplete(_ => system.terminate())
  }

}
