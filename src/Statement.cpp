
#include "Statement.h"
#include "Database.h"

Statement::Statement(const std::string &query, Database &db)
    : m_db(db), m_query(query), m_stmt(nullptr) {
  prepare();
}

Statement::~Statement() {
  if (m_stmt) {
    finalize();
  }
}

void Statement::prepare() {
  if (m_stmt) {
    finalize();
  }

  if (sqlite3_prepare_v2(m_db.getPtr(), m_query.c_str(), -1, &m_stmt,
                         nullptr) != SQLITE_OK) {
    throw std::runtime_error("Failed to prepare statement");
  }
}

void Statement::bindSingleArgument(int index, const std::string &value) {
  if (sqlite3_bind_text(m_stmt, index, value.c_str(), -1, SQLITE_STATIC) !=
      SQLITE_OK) {
    throw std::runtime_error("Failed to bind text");
  }
}

void Statement::bindArguments(std::vector<std::string> &args) {
  int i = 0;
  for (auto &arg : args)
    bindSingleArgument(i++, arg);
}

bool Statement::step() {
  int result = sqlite3_step(m_stmt);
  if (result == SQLITE_ROW) {
    return true;
  } else if (result == SQLITE_DONE) {
    return false;
  } else {
    throw std::runtime_error("Failed to step through statement");
  }
}

std::string Statement::getColumnValue(int column) {
  const unsigned char *text = sqlite3_column_text(m_stmt, column);
  if (text) {
    return std::string((const char *)text);
  }

  throw std::runtime_error("Failed to get column value at index " +
                           std::to_string(column) +
                           sqlite3_errmsg(m_db.getPtr()));
}

void Statement::reset() {
  if (sqlite3_reset(m_stmt) != SQLITE_OK) {
    throw std::runtime_error("Failed to reset statement");
  }
}

void Statement::finalize() { sqlite3_finalize(m_stmt); }
