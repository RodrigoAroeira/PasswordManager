
#include "Database.h"
#include "Statement.h"
#include <sqlite3.h>
#include <string>

class Statement;

Database::Database(const std::string &path, bool openNow = true)
    : m_path(path) {
  if (openNow)
    this->open();
}

bool Database::open() {
  return sqlite3_open(m_path.c_str(), &m_db) == SQLITE_OK;
}

bool Database::execute(const std::string &query) {
  char *errMsg = nullptr;
  Statement stmt(query, *this);

  stmt.prepare();
  if (sqlite3_exec(m_db, query.c_str(), nullptr, nullptr, &errMsg) !=
      SQLITE_OK) {
    std::string error = errMsg ? errMsg : "Unknown error";
    sqlite3_free(errMsg);
    throw std::runtime_error("Failed to execute query: " + query + ": " +
                             error);
  }
  return true;
}

void Database::addUser(const User &user) {}

void Database::addUser(const std::string &login, const std::string &password) {
  addUser(User(login, password, *this));
}

void Database::close() { sqlite3_close(m_db); }
