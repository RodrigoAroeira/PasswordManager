#pragma once

#include <sqlite3.h>
#include <string>

#include "User.h"

class User;

class Database {
public:
  Database(const std::string &path, bool openNow);

  ~Database();

  bool open();

  bool execute(const std::string &query);

  bool isOpen();

  void close();

  void addUser(const User &user);

  void addUser(const std::string &login, const std::string &password);

  sqlite3 *getPtr();

private:
  sqlite3 *m_db;
  std::string m_path;
};
