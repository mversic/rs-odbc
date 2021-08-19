#include <stdio.h>
#include <stdlib.h>
#include <sql.h>
#include <sqlext.h>
#include <string.h>

#define MAX_COL_NAME_LEN  256
#define MAX_COLS 5

void CHECK_ERROR(int retcode, char* nesto, SQLHANDLE handle, int handle_type) {
    if (retcode == 1) {
        printf("KITA");
    }
    if (!SQL_SUCCEEDED(retcode)) {
        printf("Not succeeded %d", retcode);
    }
}

int main () {

    SQLHENV   henv  = SQL_NULL_HENV;   // Environment
    SQLHDBC   hdbc  = SQL_NULL_HDBC;   // Connection handle
    SQLHSTMT  hstmt = SQL_NULL_HSTMT;  // Statement handle
    SQLRETURN retcode;

    SQLCHAR *      ColumnName[MAX_COLS];
    SQLSMALLINT    ColumnNameLen[MAX_COLS];
    SQLSMALLINT    ColumnDataType[MAX_COLS];
    SQLULEN        ColumnDataSize[MAX_COLS];
    SQLSMALLINT    ColumnDataDigits[MAX_COLS];
    SQLSMALLINT    ColumnDataNullable[MAX_COLS];
    SQLCHAR *      ColumnData[MAX_COLS];
    SQLLEN         ColumnDataLen[MAX_COLS];
    SQLSMALLINT    i,j;

    SQLCHAR statement[]="SELECT id, first_name, last_name FROM People ORDER BY 1, 3, 2";
    SQLSMALLINT numCols;

    // Initialise buffers
    for (i=0;i<MAX_COLS;i++) {
        ColumnName[i]=NULL;
        ColumnData[i]=NULL;
    }

    retcode = SQLAllocHandle(SQL_HANDLE_ENV, SQL_NULL_HANDLE, &henv);
    retcode = SQLSetEnvAttr(henv, SQL_ATTR_ODBC_VERSION, (SQLPOINTER*)SQL_OV_ODBC3, 0);
    retcode = SQLAllocHandle(SQL_HANDLE_DBC, henv, &hdbc);
    retcode=SQLDriverConnect(hdbc, NULL, "DSN=MariaDB;Database=rs_odbc_test;", SQL_NTS, NULL, 0, NULL, SQL_DRIVER_COMPLETE);
    printf("%d", retcode);
    retcode = SQLAllocHandle(SQL_HANDLE_STMT, hdbc, &hstmt);
    printf("%d", retcode);

    // Prepare Statement (my change to free format input by user)
    retcode = SQLPrepare (hstmt, statement, strlen(statement));

    // Retrieve number of columns
    retcode = SQLNumResultCols (hstmt, &numCols);

    printf ("Number of Result Columns %i\n", numCols);

    // Loop round number of columns using SQLDescribeCol to get info about
    // the column, followed by SQLBindCol to bind the column to a data area
    for (i=0;i<numCols;i++) {
        ColumnName[i] = (SQLCHAR *) malloc (MAX_COL_NAME_LEN);
        retcode = SQLDescribeCol (
                    hstmt,                    // Select Statement (Prepared)
                    i+1,                      // Columnn Number
                    ColumnName[i],            // Column Name (returned)
                    MAX_COL_NAME_LEN,         // size of Column Name buffer
                    &ColumnNameLen[i],        // Actual size of column name
                    &ColumnDataType[i],       // SQL Data type of column
                    &ColumnDataSize[i],       // Data size of column in table
                    &ColumnDataDigits[i],     // Number of decimal digits
                    &ColumnDataNullable[i]);  // Whether column nullable

        CHECK_ERROR(retcode, "SQLDescribeCol()", hstmt, SQL_HANDLE_STMT);

        // Display column data
        printf("\nColumn : %i\n", i+1);
        printf("Column Name : %s\n  Column Name Len : %i\n  SQL Data Type : %i\n  Data Size : %i\n  DecimalDigits : %i\n  Nullable %i\n",
                 ColumnName[i], (int)ColumnNameLen[i], (int)ColumnDataType[i],
                 (int)ColumnDataSize[i], (int)ColumnDataDigits[i],
                 (int)ColumnDataNullable[i]);

        // Bind column, changing SQL data type to C data type
        // (assumes INT and VARCHAR for now)
        ColumnData[i] = (SQLCHAR *) malloc (ColumnDataSize[i]+1);
        switch (ColumnDataType[i]) {
            case SQL_INTEGER:
                ColumnDataType[i]=SQL_C_LONG;
            break;
            case SQL_VARCHAR:
                ColumnDataType[i]=SQL_C_CHAR;
            break;
        }

        retcode = SQLBindCol (hstmt,                  // Statement handle
                              i+1,                    // Column number
                              ColumnDataType[i],      // C Data Type
                              ColumnData[i],          // Data buffer
                              ColumnDataSize[i],      // Size of Data Buffer
                              &ColumnDataLen[i]); // Size of data returned

        CHECK_ERROR(retcode, "SQLBindCol()", hstmt, SQL_HANDLE_STMT);
    }

    //// Fetch records
    //printf ("\nRecords ...\n\n");
    //retcode = SQLExecute (hstmt);
    //CHECK_ERROR(retcode, "SQLExecute()", hstmt, SQL_HANDLE_STMT);

    //printf ("\n  Data Records\n  ------------\n");
    //for (i=0; ; i++) {
    //    retcode = SQLFetch(hstmt);

    //    //No more data?
    //    if (retcode == SQL_NO_DATA) {
    //        break;
    //    }

    //    CHECK_ERROR(retcode, "SQLFetch()", hstmt, SQL_HANDLE_STMT);

    //    //Display it
    //    printf ("\nRecord %i \n", i+1);
    //    for (j=0;j<numCols;j++) {
    //        printf("Column %s : ", ColumnName[j]);
    //        if (ColumnDataType[j]==SQL_INTEGER) {
    //            printf(" %i\n", (int) *ColumnData[j]);
    //        } else {
    //            printf(" %s\n", rtrim(ColumnData[j], ' '));
    //        }
    //    }
    //}

    //for (i=0;i<numCols;i++) {
    //    if (ColumnName[i]!=NULL) free (ColumnName[i]);
    //    if (ColumnData[i]!=NULL) free (ColumnData[i]);
    //}

    //printf ("\nComplete.\n");

    //// Free handles
    //// Statement
    //if (hstmt != SQL_NULL_HSTMT)
    //    SQLFreeHandle(SQL_HANDLE_STMT, hstmt);

    //// Connection
    //if (hdbc != SQL_NULL_HDBC) {
    //    SQLDisconnect(hdbc);
    //    SQLFreeHandle(SQL_HANDLE_DBC, hdbc);
    //}

    //// Environment
    //if (henv != SQL_NULL_HENV)
    //    SQLFreeHandle(SQL_HANDLE_ENV, henv);

    //return 0;
}
