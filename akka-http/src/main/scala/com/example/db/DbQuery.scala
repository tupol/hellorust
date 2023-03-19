package com.example.db

import com.zaxxer.hikari.HikariDataSource

import java.sql.Connection
import scala.util.Try

object DbQuery {

  val sql = """select * from userinfo(?, ?, ?);""".stripMargin

  def userInfo(username: String, pool: HikariConnectionPool): Try[UserInfo] =
    for {
      conn <- pool.connection
      stmt <- Try { conn.prepareStatement(sql) }
      _    <- Try { stmt.setString(1, username) }
      _    <- Try { stmt.setString(2, "empty") }
      _    <- Try { stmt.setString(3, "empty") }
      res  <- Try { stmt.executeQuery() }
      _    <- Try { res.next() }
      usr <- Try {
              UserInfo(
                username = res.getString(1),
                password = res.getString(2),
                salt = res.getString(3),
                minLevelOfAssurance = res.getInt(4),
                maxLevelOfAssurance = res.getInt(5),
                authenticationMeansRef = res.getString(6),
                authenticationMeansState = res.getString(7),
                accountLockedTimeSeconds = res.getInt(8),
                name = res.getString(10),
                emailAddress = res.getString(11),
                typeUser = res.getString(12),
                firstName = res.getString(13),
                lastName = res.getString(14),
                userTechId = res.getString(15)
              )
            }
      _ <- Try { res.close(); stmt.close(); conn.close() }
    } yield usr

}

trait ConnectionPool {
  def connection: Try[Connection]
  def close: Unit
}

case class HikariConnectionPool private (datasource: HikariDataSource) extends ConnectionPool {
  override def connection: Try[Connection] = Try(datasource.getConnection)

  override def close: Unit = datasource.close
}
