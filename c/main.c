#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include "clib_openiap.h"

#define INPUT_SIZE 256

void print_help() {
    printf("Available commands:\n");
    printf("  ?         : Help\n");
    printf("  connect   : Reconnect to server\n");
    printf("  info      : Log an info message\n");
    printf("  warn      : Log a warning message\n");
    printf("  error     : Log an error message\n");
    printf("  o         : Toggle observable gauge 'test_f64'\n");
    printf("  q         : Execute a query on 'entities' collection\n");
    printf("  quit      : Exit the CLI\n");
}
int main(void) {
    char input[INPUT_SIZE];
    bool gauge_active = false;

    struct ClientWrapper *client = create_client();
    if (client == NULL) {
        fprintf(stderr, "Error: Failed to create client.\n");
        return 1;
    }
    
    const char *server_address = "";
    struct ConnectResponseWrapper *conn_resp = client_connect(client, server_address);
    if (conn_resp == NULL) {
        fprintf(stderr, "Error: client_connect returned NULL.\n");
        return 1;
    }
    
    if (!conn_resp->success) {
        fprintf(stderr, "Connection failed: %s\n", conn_resp->error);
        return 1;
    } else {
        printf("Connected successfully! Request ID: %d\n", conn_resp->request_id);
    }
    free_connect_response(conn_resp);

    print_help();

    while (1) {
        printf("> ");
        if (fgets(input, sizeof(input), stdin) == NULL) {
            break;
        }
        input[strcspn(input, "\n")] = '\0';

        if (strcmp(input, "quit") == 0) {
            break;
        } else if (strcmp(input, "?") == 0) {
            print_help();
        } else if (strcmp(input, "connect") == 0) {
            conn_resp = client_connect(client, server_address);
            if (conn_resp == NULL) {
                printf("Error: client_connect returned NULL.\n");
            } else if (!conn_resp->success) {
                printf("Connection failed: %s\n", conn_resp->error);
            } else {
                printf("Connected successfully! Request ID: %d\n", conn_resp->request_id);
            }
            free_connect_response(conn_resp);
        } else if (strcmp(input, "info") == 0) {
            info("This is an info message from the CLI.");
        } else if (strcmp(input, "warn") == 0) {
            warn("This is a warning message from the CLI.");
        } else if (strcmp(input, "error") == 0) {
            error("This is an error message from the CLI.");
        } else if (strcmp(input, "o") == 0) {
            if (!gauge_active) {
                // Set the observable gauge "test_f64" to 42.7.
                set_f64_observable_gauge("test_f64", 42.7, "test observable gauge");
                printf("Observable gauge 'test_f64' set to 42.7.\n");
                gauge_active = true;
            } else {
                // Disable the observable gauge.
                disable_observable_gauge("test_f64");
                printf("Observable gauge 'test_f64' disabled.\n");
                gauge_active = false;
            }
        } else if (strcmp(input, "q") == 0) {
            // Build a query request for the "entities" collection.
            QueryRequestWrapper req;
            req.collectionname = "entities";
            req.query = "{}";
            req.projection = "{ \"name\": 1 }";
            req.orderby = NULL;
            req.queryas = NULL;
            req.explain = false;
            req.skip = 0;
            req.top = 0;
            req.request_id = 1; // sample request id

            struct QueryResponseWrapper *query_resp = query(client, &req);
            if (query_resp == NULL) {
                printf("Error: query returned NULL.\n");
            } else {
                if (!query_resp->success) {
                    printf("Query failed: %s\n", query_resp->error);
                } else {
                    printf("Query succeeded. Results: %s\n", query_resp->results);
                }
                free_query_response(query_resp);
            }
        } else {
            printf("Unknown command: '%s'\n", input);
        }
    }

    client_disconnect(client);
    free_client(client);
    
    printf("Exiting CLI.\n");
    return 0;
}
