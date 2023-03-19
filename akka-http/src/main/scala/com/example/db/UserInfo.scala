package com.example.db

case class UserInfo(
  username: String,
  password: String,
  salt: String,
  minLevelOfAssurance: Int,
  maxLevelOfAssurance: Int,
  authenticationMeansRef: String,
  authenticationMeansState: String,
  accountLockedTimeSeconds: Int,
  name: String,
  emailAddress: String,
  typeUser: String,
  firstName: String,
  lastName: String,
  userTechId: String
)
