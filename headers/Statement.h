#pragma once

#include <sqlite3.h>
#include <stdexcept>
#include <string>
#include <vector>

#include "Database.h"

class Database;

class Statement {
public:
  Statement(const std::string &query, Database &db);

  ~Statement();

  void prepare();

  void bindSingleArgument(int index, const std::string &value);

  void bindArguments(std::vector<std::string> &args);

  bool step();

  std::string getColumnValue(int column);

  void reset();

  void finalize();

private:
  Database &m_db;
  sqlite3_stmt *m_stmt;
  std::string m_query;
};
