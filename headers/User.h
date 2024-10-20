#pragma once
#include <string>

#include "Database.h"

class Database;

class User {
public:
  User(const std::string &login, const std::string &password,
       const Database &db);

  const std::string &getLogin() const;

  const std::string &getPassHash() const;

private:
  std::string m_login;
  std::string m_password;
  Database &m_db;
};
