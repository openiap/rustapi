typedef struct Option_Client {
  int some_field; // Example field
} Option_Client;


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Option_Client Option_Client;

typedef struct UserWrapper {
  const char *id;
  const char *name;
  const char *username;
  const char *email;
  const char *const *roles;
  int32_t roles_len;
} UserWrapper;

/**
 * A wrapper for the client library.
 * This struct is used to hold the client instance and the runtime instance.
 */
typedef struct ClientWrapper {
  bool success;
  const char *error;
  struct Option_Client client;
} ClientWrapper;

/**
 * QueryResponseWrapper is a wrapper for the QueryResponse struct.
 */
typedef struct QueryResponseWrapper {
  bool success;
  const char *results;
  const char *error;
  int32_t request_id;
} QueryResponseWrapper;

/**
 * QueryRequestWrapper is a wrapper for the QuQueryResponseWrappereryRequest struct.
 */
typedef struct QueryRequestWrapper {
  const char *collectionname;
  const char *query;
  const char *projection;
  const char *orderby;
  const char *queryas;
  bool explain;
  int32_t skip;
  int32_t top;
  int32_t request_id;
} QueryRequestWrapper;

/**
 * QueryCallback is a callback function for the query_async function.
 */
typedef void (*QueryCallback)(struct QueryResponseWrapper *wrapper);

/**
 * CustomCommandResponseWrapper is a C-compatible wrapper for CustomCommandResponse.
 */
typedef struct CustomCommandResponseWrapper {
  bool success;
  const char *result;
  const char *error;
  int32_t request_id;
} CustomCommandResponseWrapper;

/**
 * CustomCommandRequestWrapper is a C-compatible wrapper for CustomCommandRequest.
 */
typedef struct CustomCommandRequestWrapper {
  const char *command;
  const char *id;
  const char *name;
  const char *data;
  int32_t request_id;
} CustomCommandRequestWrapper;

typedef void (*CustomCommandCallback)(struct CustomCommandResponseWrapper *wrapper);

/**
 * A wrapper for the client library.
 * This struct is used to hold the client instance and the runtime instance.
 */
typedef struct ConnectResponseWrapper {
  bool success;
  const char *error;
  int32_t request_id;
} ConnectResponseWrapper;

typedef void (*ConnectCallback)(struct ConnectResponseWrapper *wrapper);

typedef struct SigninResponseWrapper {
  bool success;
  const char *jwt;
  const char *error;
  int32_t request_id;
} SigninResponseWrapper;

typedef struct SigninRequestWrapper {
  const char *username;
  const char *password;
  const char *jwt;
  const char *agent;
  const char *version;
  bool longtoken;
  bool validateonly;
  bool ping;
  int32_t request_id;
} SigninRequestWrapper;

typedef void (*SigninCallback)(struct SigninResponseWrapper *wrapper);

typedef struct ListCollectionsResponseWrapper {
  bool success;
  const char *results;
  const char *error;
  int32_t request_id;
} ListCollectionsResponseWrapper;

typedef void (*ListCollectionsCallback)(struct ListCollectionsResponseWrapper *wrapper);

typedef struct CreateCollectionResponseWrapper {
  bool success;
  const char *error;
  int32_t request_id;
} CreateCollectionResponseWrapper;

typedef struct ColCollationWrapper {
  const char *locale;
  bool case_level;
  const char *case_first;
  int32_t strength;
  bool numeric_ordering;
  const char *alternate;
  const char *max_variable;
  bool backwards;
} ColCollationWrapper;

typedef struct ColTimeseriesWrapper {
  const char *time_field;
  const char *meta_field;
  const char *granularity;
} ColTimeseriesWrapper;

typedef struct CreateCollectionRequestWrapper {
  const char *collectionname;
  struct ColCollationWrapper *collation;
  struct ColTimeseriesWrapper *timeseries;
  int32_t expire_after_seconds;
  bool change_stream_pre_and_post_images;
  bool capped;
  int32_t max;
  int32_t size;
  int32_t request_id;
} CreateCollectionRequestWrapper;

typedef void (*CreateCollectionCallback)(struct CreateCollectionResponseWrapper *wrapper);

typedef struct DropCollectionResponseWrapper {
  bool success;
  const char *error;
  int32_t request_id;
} DropCollectionResponseWrapper;

typedef void (*DropCollectionCallback)(struct DropCollectionResponseWrapper *wrapper);

typedef struct GetIndexesResponseWrapper {
  bool success;
  const char *results;
  const char *error;
  int32_t request_id;
} GetIndexesResponseWrapper;

typedef void (*GetIndexesCallback)(struct GetIndexesResponseWrapper *wrapper);

typedef struct CreateIndexResponseWrapper {
  bool success;
  const char *error;
  int32_t request_id;
} CreateIndexResponseWrapper;

typedef struct CreateIndexRequestWrapper {
  const char *collectionname;
  const char *index;
  const char *options;
  const char *name;
  int32_t request_id;
} CreateIndexRequestWrapper;

typedef void (*CreateIndexCallback)(struct CreateIndexResponseWrapper *wrapper);

typedef struct DropIndexResponseWrapper {
  bool success;
  const char *error;
  int32_t request_id;
} DropIndexResponseWrapper;

typedef void (*DropIndexCallback)(struct DropIndexResponseWrapper *wrapper);

typedef struct AggregateResponseWrapper {
  bool success;
  const char *results;
  const char *error;
  int32_t request_id;
} AggregateResponseWrapper;

typedef struct AggregateRequestWrapper {
  const char *collectionname;
  const char *aggregates;
  const char *queryas;
  const char *hint;
  bool explain;
  int32_t request_id;
} AggregateRequestWrapper;

typedef void (*AggregateCallback)(struct AggregateResponseWrapper *wrapper);

typedef struct CountResponseWrapper {
  bool success;
  int32_t result;
  const char *error;
  int32_t request_id;
} CountResponseWrapper;

typedef struct CountRequestWrapper {
  const char *collectionname;
  const char *query;
  const char *queryas;
  bool explain;
  int32_t request_id;
} CountRequestWrapper;

typedef void (*CountCallback)(struct CountResponseWrapper *wrapper);

typedef struct DistinctResponseWrapper {
  bool success;
  const char **results;
  const char *error;
  int32_t results_len;
  int32_t request_id;
} DistinctResponseWrapper;

typedef struct DistinctRequestWrapper {
  const char *collectionname;
  const char *field;
  const char *query;
  const char *queryas;
  bool explain;
  int32_t request_id;
} DistinctRequestWrapper;

typedef void (*DistinctCallback)(struct DistinctResponseWrapper *wrapper);

typedef struct InsertOneResponseWrapper {
  bool success;
  const char *result;
  const char *error;
  int32_t request_id;
} InsertOneResponseWrapper;

typedef struct InsertOneRequestWrapper {
  const char *collectionname;
  const char *item;
  int32_t w;
  bool j;
  int32_t request_id;
} InsertOneRequestWrapper;

typedef void (*InsertOneCallback)(struct InsertOneResponseWrapper *wrapper);

typedef struct InsertManyResponseWrapper {
  bool success;
  const char *results;
  const char *error;
  int32_t request_id;
} InsertManyResponseWrapper;

typedef struct InsertManyRequestWrapper {
  const char *collectionname;
  const char *items;
  int32_t w;
  bool j;
  bool skipresults;
  int32_t request_id;
} InsertManyRequestWrapper;

typedef void (*InsertManyCallback)(struct InsertManyResponseWrapper *wrapper);

typedef struct UpdateOneResponseWrapper {
  bool success;
  const char *result;
  const char *error;
  int32_t request_id;
} UpdateOneResponseWrapper;

typedef struct UpdateOneRequestWrapper {
  const char *collectionname;
  const char *item;
  int32_t w;
  bool j;
  int32_t request_id;
} UpdateOneRequestWrapper;

typedef void (*UpdateOneCallback)(struct UpdateOneResponseWrapper *wrapper);

typedef struct InsertOrUpdateOneResponseWrapper {
  bool success;
  const char *result;
  const char *error;
  int32_t request_id;
} InsertOrUpdateOneResponseWrapper;

typedef struct InsertOrUpdateOneRequestWrapper {
  const char *collectionname;
  const char *uniqeness;
  const char *item;
  int32_t w;
  bool j;
  int32_t request_id;
} InsertOrUpdateOneRequestWrapper;

typedef void (*InsertOrUpdateOneCallback)(struct InsertOrUpdateOneResponseWrapper *wrapper);

typedef struct DeleteOneResponseWrapper {
  bool success;
  int32_t affectedrows;
  const char *error;
  int32_t request_id;
} DeleteOneResponseWrapper;

typedef struct DeleteOneRequestWrapper {
  const char *collectionname;
  const char *id;
  bool recursive;
  int32_t request_id;
} DeleteOneRequestWrapper;

typedef void (*DeleteOneCallback)(struct DeleteOneResponseWrapper *wrapper);

typedef struct DeleteManyResponseWrapper {
  bool success;
  int32_t affectedrows;
  const char *error;
  int32_t request_id;
} DeleteManyResponseWrapper;

typedef struct DeleteManyRequestWrapper {
  const char *collectionname;
  const char *query;
  bool recursive;
  const char *const *ids;
  int32_t request_id;
} DeleteManyRequestWrapper;

typedef void (*DeleteManyCallback)(struct DeleteManyResponseWrapper *wrapper);

typedef struct DownloadResponseWrapper {
  bool success;
  const char *filename;
  const char *error;
  int32_t request_id;
} DownloadResponseWrapper;

typedef struct DownloadRequestWrapper {
  const char *collectionname;
  const char *id;
  const char *folder;
  const char *filename;
  int32_t request_id;
} DownloadRequestWrapper;

typedef void (*DownloadCallback)(struct DownloadResponseWrapper *wrapper);

typedef struct UploadResponseWrapper {
  bool success;
  const char *id;
  const char *error;
  int32_t request_id;
} UploadResponseWrapper;

typedef struct UploadRequestWrapper {
  const char *filepath;
  const char *filename;
  const char *mimetype;
  const char *metadata;
  const char *collectionname;
  int32_t request_id;
} UploadRequestWrapper;

typedef void (*UploadCallback)(struct UploadResponseWrapper *wrapper);

typedef struct WatchResponseWrapper {
  bool success;
  const char *watchid;
  const char *error;
  int32_t request_id;
} WatchResponseWrapper;

typedef struct WatchRequestWrapper {
  const char *collectionname;
  const char *paths;
  int32_t request_id;
} WatchRequestWrapper;

/**
 * WatchEventWrapper is a wrapper for the WatchEvent struct.
 */
typedef struct WatchEventWrapper {
  const char *id;
  const char *operation;
  const char *document;
  int32_t request_id;
} WatchEventWrapper;

typedef void (*WatchCallback)(struct WatchResponseWrapper *wrapper);

typedef void (*WatchEventCallback)(struct WatchEventWrapper*);

typedef struct UnWatchResponseWrapper {
  bool success;
  const char *error;
  int32_t request_id;
} UnWatchResponseWrapper;

typedef struct RegisterQueueResponseWrapper {
  bool success;
  const char *queuename;
  const char *error;
  int32_t request_id;
} RegisterQueueResponseWrapper;

typedef struct RegisterQueueRequestWrapper {
  const char *queuename;
  int32_t request_id;
} RegisterQueueRequestWrapper;

typedef struct QueueEventWrapper {
  const char *queuename;
  const char *correlation_id;
  const char *replyto;
  const char *routingkey;
  const char *exchangename;
  const char *data;
  int32_t request_id;
} QueueEventWrapper;

typedef const char *(*QueueEventCallback)(struct QueueEventWrapper*);

typedef struct RegisterExchangeResponseWrapper {
  bool success;
  const char *queuename;
  const char *error;
  int32_t request_id;
} RegisterExchangeResponseWrapper;

typedef struct RegisterExchangeRequestWrapper {
  const char *exchangename;
  const char *algorithm;
  const char *routingkey;
  bool addqueue;
  int32_t request_id;
} RegisterExchangeRequestWrapper;

typedef void (*ExchangeEventCallback)(struct QueueEventWrapper*);

typedef struct QueueMessageResponseWrapper {
  bool success;
  const char *error;
} QueueMessageResponseWrapper;

typedef struct QueueMessageRequestWrapper {
  const char *queuename;
  const char *correlation_id;
  const char *replyto;
  const char *routingkey;
  const char *exchangename;
  const char *data;
  bool striptoken;
  int32_t expiration;
  int32_t request_id;
} QueueMessageRequestWrapper;

typedef struct UnRegisterQueueResponseWrapper {
  bool success;
  const char *error;
} UnRegisterQueueResponseWrapper;

typedef struct WorkitemFileWrapper {
  const char *filename;
  const char *id;
  bool compressed;
} WorkitemFileWrapper;

typedef struct WorkitemWrapper {
  const char *id;
  const char *name;
  const char *payload;
  int32_t priority;
  uint64_t nextrun;
  uint64_t lastrun;
  const struct WorkitemFileWrapper *const *files;
  int32_t files_len;
  const char *state;
  const char *wiq;
  const char *wiqid;
  int32_t retries;
  const char *username;
  const char *success_wiqid;
  const char *failed_wiqid;
  const char *success_wiq;
  const char *failed_wiq;
  const char *errormessage;
  const char *errorsource;
  const char *errortype;
} WorkitemWrapper;

typedef struct PushWorkitemResponseWrapper {
  bool success;
  const char *error;
  const struct WorkitemWrapper *workitem;
  int32_t request_id;
} PushWorkitemResponseWrapper;

typedef struct PushWorkitemRequestWrapper {
  const char *wiq;
  const char *wiqid;
  const char *name;
  const char *payload;
  uint64_t nextrun;
  const char *success_wiqid;
  const char *failed_wiqid;
  const char *success_wiq;
  const char *failed_wiq;
  int32_t priority;
  const struct WorkitemFileWrapper *const *files;
  int32_t files_len;
  int32_t request_id;
} PushWorkitemRequestWrapper;

typedef struct PopWorkitemResponseWrapper {
  bool success;
  const char *error;
  const struct WorkitemWrapper *workitem;
  int32_t request_id;
} PopWorkitemResponseWrapper;

typedef struct PopWorkitemRequestWrapper {
  const char *wiq;
  const char *wiqid;
  int32_t request_id;
} PopWorkitemRequestWrapper;

typedef struct UpdateWorkitemResponseWrapper {
  bool success;
  const char *error;
  const struct WorkitemWrapper *workitem;
  int32_t request_id;
} UpdateWorkitemResponseWrapper;

typedef struct UpdateWorkitemRequestWrapper {
  const struct WorkitemWrapper *workitem;
  bool ignoremaxretries;
  const struct WorkitemFileWrapper *const *files;
  int32_t files_len;
  int32_t request_id;
} UpdateWorkitemRequestWrapper;

typedef struct DeleteWorkitemResponseWrapper {
  bool success;
  const char *error;
  int32_t request_id;
} DeleteWorkitemResponseWrapper;

typedef struct DeleteWorkitemRequestWrapper {
  const char *id;
  int32_t request_id;
} DeleteWorkitemRequestWrapper;

typedef struct ClientEventResponseWrapper {
  bool success;
  const char *eventid;
  const char *error;
} ClientEventResponseWrapper;

typedef struct ClientEventWrapper {
  const char *event;
  const char *reason;
} ClientEventWrapper;

typedef void (*ClientEventCallback)(struct ClientEventWrapper*);

typedef struct OffClientEventResponseWrapper {
  bool success;
  const char *error;
} OffClientEventResponseWrapper;

typedef struct RpcResponseWrapper {
  bool success;
  const char *result;
  const char *error;
  int32_t request_id;
} RpcResponseWrapper;

typedef void (*RpcResponseCallback)(struct RpcResponseWrapper*);

/**
 * InvokeOpenRPAResponseWrapper is a wrapper for the InvokeOpenRpaRequest struct.
 */
typedef struct InvokeOpenRPAResponseWrapper {
  bool success;
  const char *result;
  const char *error;
  int32_t request_id;
} InvokeOpenRPAResponseWrapper;

/**
 * InvokeOpenRPARequestWrapper is a wrapper for the QuQueryResponseWrappereryRequest struct.
 */
typedef struct InvokeOpenRPARequestWrapper {
  const char *robotid;
  const char *workflowid;
  const char *payload;
  bool rpc;
  int32_t request_id;
} InvokeOpenRPARequestWrapper;

void error(const char *message);

void info(const char *message);

void warn(const char *message);

void debug(const char *message);

void trace(const char *message);

void set_f64_observable_gauge(const char *name, double value, const char *description);

void set_u64_observable_gauge(const char *name, uint64_t value, const char *description);

void set_i64_observable_gauge(const char *name, int64_t value, const char *description);

void disable_observable_gauge(const char *name);

/**
 * Return currentlly signed in user
 */
const struct UserWrapper *client_user(struct ClientWrapper *client);

/**
 * Free the user wrapper
 */
void free_user(struct UserWrapper *user);

struct QueryResponseWrapper *query(struct ClientWrapper *client,
                                   struct QueryRequestWrapper *options);

void query_async(struct ClientWrapper *client,
                 struct QueryRequestWrapper *options,
                 QueryCallback callback);

void free_query_response(struct QueryResponseWrapper *response);

struct CustomCommandResponseWrapper *custom_command(struct ClientWrapper *client,
                                                    struct CustomCommandRequestWrapper *options,
                                                    int32_t timeout);

void custom_command_async(struct ClientWrapper *client,
                          struct CustomCommandRequestWrapper *options,
                          CustomCommandCallback callback,
                          int32_t timeout);

void free_custom_command_response(struct CustomCommandResponseWrapper *response);

void enable_tracing(const char *rust_log, const char *tracing);

void disable_tracing(void);

struct ClientWrapper *create_client(void);

struct ConnectResponseWrapper *client_connect(struct ClientWrapper *client_wrap,
                                              const char *server_address);

void connect_async(struct ClientWrapper *client,
                   const char *server_address,
                   int32_t request_id,
                   ConnectCallback callback);

int32_t client_get_default_timeout(struct ClientWrapper *client_wrap);

void client_set_default_timeout(struct ClientWrapper *client_wrap, int32_t timeout);

const char *client_get_state(struct ClientWrapper *client_wrap);

void client_set_agent_name(struct ClientWrapper *client_wrap, const char *agent_name);

void client_set_agent_version(struct ClientWrapper *client_wrap, const char *agent_version);

void free_connect_response(struct ConnectResponseWrapper *response);

void client_disconnect(struct ClientWrapper *client_wrap);

void free_client(struct ClientWrapper *response);

struct SigninResponseWrapper *signin(struct ClientWrapper *client,
                                     struct SigninRequestWrapper *options);

void signin_async(struct ClientWrapper *client,
                  struct SigninRequestWrapper *options,
                  SigninCallback callback);

void free_signin_response(struct SigninResponseWrapper *response);

struct ListCollectionsResponseWrapper *list_collections(struct ClientWrapper *client,
                                                        bool includehist);

void list_collections_async(struct ClientWrapper *client,
                            bool includehist,
                            int32_t request_id,
                            ListCollectionsCallback callback);

void free_list_collections_response(struct ListCollectionsResponseWrapper *response);

struct CreateCollectionResponseWrapper *create_collection(struct ClientWrapper *client,
                                                          struct CreateCollectionRequestWrapper *options);

void create_collection_async(struct ClientWrapper *client,
                             struct CreateCollectionRequestWrapper *options,
                             CreateCollectionCallback callback);

void free_create_collection_response(struct CreateCollectionResponseWrapper *response);

struct DropCollectionResponseWrapper *drop_collection(struct ClientWrapper *client,
                                                      const char *collectionname);

void drop_collection_async(struct ClientWrapper *client,
                           const char *collectionname,
                           int32_t request_id,
                           DropCollectionCallback callback);

void free_drop_collection_response(struct DropCollectionResponseWrapper *response);

struct GetIndexesResponseWrapper *get_indexes(struct ClientWrapper *client,
                                              const char *collectionname);

void get_indexes_async(struct ClientWrapper *client,
                       const char *collectionname,
                       int32_t request_id,
                       GetIndexesCallback callback);

void free_get_indexes_response(struct GetIndexesResponseWrapper *response);

struct CreateIndexResponseWrapper *create_index(struct ClientWrapper *client,
                                                struct CreateIndexRequestWrapper *options);

void create_index_async(struct ClientWrapper *client,
                        struct CreateIndexRequestWrapper *options,
                        CreateIndexCallback callback);

void free_create_index_response(struct CreateIndexResponseWrapper *response);

struct DropIndexResponseWrapper *drop_index(struct ClientWrapper *client,
                                            const char *collectionname,
                                            const char *name);

void drop_index_async(struct ClientWrapper *client,
                      const char *collectionname,
                      const char *name,
                      int32_t request_id,
                      DropIndexCallback callback);

void free_drop_index_response(struct DropIndexResponseWrapper *response);

struct AggregateResponseWrapper *aggregate(struct ClientWrapper *client,
                                           struct AggregateRequestWrapper *options);

void aggregate_async(struct ClientWrapper *client,
                     struct AggregateRequestWrapper *options,
                     AggregateCallback callback);

void free_aggregate_response(struct AggregateResponseWrapper *response);

struct CountResponseWrapper *count(struct ClientWrapper *client,
                                   struct CountRequestWrapper *options);

void count_async(struct ClientWrapper *client,
                 struct CountRequestWrapper *options,
                 CountCallback callback);

void free_count_response(struct CountResponseWrapper *response);

struct DistinctResponseWrapper *distinct(struct ClientWrapper *client,
                                         struct DistinctRequestWrapper *options);

void distinct_async(struct ClientWrapper *client,
                    struct DistinctRequestWrapper *options,
                    DistinctCallback callback);

void free_distinct_response(struct DistinctResponseWrapper *response);

struct InsertOneResponseWrapper *insert_one(struct ClientWrapper *client,
                                            struct InsertOneRequestWrapper *options);

void insert_one_async(struct ClientWrapper *client,
                      struct InsertOneRequestWrapper *options,
                      InsertOneCallback callback);

void free_insert_one_response(struct InsertOneResponseWrapper *response);

struct InsertManyResponseWrapper *insert_many(struct ClientWrapper *client,
                                              struct InsertManyRequestWrapper *options);

void insert_many_async(struct ClientWrapper *client,
                       struct InsertManyRequestWrapper *options,
                       InsertManyCallback callback);

void free_insert_many_response(struct InsertManyResponseWrapper *response);

struct UpdateOneResponseWrapper *update_one(struct ClientWrapper *client,
                                            struct UpdateOneRequestWrapper *options);

void update_one_async(struct ClientWrapper *client,
                      struct UpdateOneRequestWrapper *options,
                      UpdateOneCallback callback);

void free_update_one_response(struct UpdateOneResponseWrapper *response);

struct InsertOrUpdateOneResponseWrapper *insert_or_update_one(struct ClientWrapper *client,
                                                              struct InsertOrUpdateOneRequestWrapper *options);

void insert_or_update_one_async(struct ClientWrapper *client,
                                struct InsertOrUpdateOneRequestWrapper *options,
                                InsertOrUpdateOneCallback callback);

void free_insert_or_update_one_response(struct InsertOrUpdateOneResponseWrapper *response);

struct DeleteOneResponseWrapper *delete_one(struct ClientWrapper *client,
                                            struct DeleteOneRequestWrapper *options);

void delete_one_async(struct ClientWrapper *client,
                      struct DeleteOneRequestWrapper *options,
                      DeleteOneCallback callback);

void free_delete_one_response(struct DeleteOneResponseWrapper *response);

struct DeleteManyResponseWrapper *delete_many(struct ClientWrapper *client,
                                              struct DeleteManyRequestWrapper *options);

void delete_many_async(struct ClientWrapper *client,
                       struct DeleteManyRequestWrapper *options,
                       DeleteManyCallback callback);

void free_delete_many_response(struct DeleteManyResponseWrapper *response);

struct DownloadResponseWrapper *download(struct ClientWrapper *client,
                                         struct DownloadRequestWrapper *options);

void download_async(struct ClientWrapper *client,
                    struct DownloadRequestWrapper *options,
                    DownloadCallback callback);

void free_download_response(struct DownloadResponseWrapper *response);

struct UploadResponseWrapper *upload(struct ClientWrapper *client,
                                     struct UploadRequestWrapper *options);

void upload_async(struct ClientWrapper *client,
                  struct UploadRequestWrapper *options,
                  UploadCallback callback);

void free_upload_response(struct UploadResponseWrapper *response);

struct WatchResponseWrapper *watch(struct ClientWrapper *client,
                                   struct WatchRequestWrapper *options);

struct WatchEventWrapper *next_watch_event(const char *watchid);

void free_watch_event(struct WatchEventWrapper *response);

void watch_async_async(struct ClientWrapper *client,
                       struct WatchRequestWrapper *options,
                       WatchCallback callback,
                       WatchEventCallback event_callback);

void free_watch_response(struct WatchResponseWrapper *response);

struct UnWatchResponseWrapper *unwatch(struct ClientWrapper *client, const char *watchid);

void unwatch_async(struct ClientWrapper *client,
                   const char *watchid,
                   int32_t request_id,
                   void (*callback)(struct UnWatchResponseWrapper*));

void free_unwatch_response(struct UnWatchResponseWrapper *response);

struct RegisterQueueResponseWrapper *register_queue(struct ClientWrapper *client,
                                                    struct RegisterQueueRequestWrapper *options);

struct RegisterQueueResponseWrapper *register_queue_async(struct ClientWrapper *client,
                                                          struct RegisterQueueRequestWrapper *options,
                                                          QueueEventCallback event_callback);

void free_register_queue_response(struct RegisterQueueResponseWrapper *response);

struct RegisterExchangeResponseWrapper *register_exchange(struct ClientWrapper *client,
                                                          struct RegisterExchangeRequestWrapper *options);

struct RegisterExchangeResponseWrapper *register_exchange_async(struct ClientWrapper *client,
                                                                struct RegisterExchangeRequestWrapper *options,
                                                                ExchangeEventCallback event_callback);

void free_register_exchange_response(struct RegisterExchangeResponseWrapper *response);

struct QueueEventWrapper *next_queue_event(const char *queuename);

void free_queue_event(struct QueueEventWrapper *response);

struct QueueMessageResponseWrapper *queue_message(struct ClientWrapper *client,
                                                  struct QueueMessageRequestWrapper *options);

void free_queue_message_response(struct QueueMessageResponseWrapper *response);

struct UnRegisterQueueResponseWrapper *unregister_queue(struct ClientWrapper *client,
                                                        const char *queuename);

void free_unregister_queue_response(struct UnRegisterQueueResponseWrapper *response);

struct PushWorkitemResponseWrapper *push_workitem(struct ClientWrapper *client,
                                                  struct PushWorkitemRequestWrapper *options);

void push_workitem_async(struct ClientWrapper *client,
                         struct PushWorkitemRequestWrapper *options,
                         void (*callback)(struct PushWorkitemResponseWrapper*));

void free_push_workitem_response(struct PushWorkitemResponseWrapper *response);

struct PopWorkitemResponseWrapper *pop_workitem(struct ClientWrapper *client,
                                                struct PopWorkitemRequestWrapper *options,
                                                const char *downloadfolder);

void pop_workitem_async(struct ClientWrapper *client,
                        struct PopWorkitemRequestWrapper *options,
                        const char *downloadfolder,
                        void (*callback)(struct PopWorkitemResponseWrapper*));

void free_pop_workitem_response(struct PopWorkitemResponseWrapper *response);

void free_workitem_file(struct WorkitemFileWrapper *file);

void free_workitem(struct WorkitemWrapper *workitem);

void pop_workitem2_async(struct ClientWrapper *_client,
                         struct PopWorkitemRequestWrapper *_options,
                         const char *_downloadfolder,
                         int32_t request_id,
                         void (*callback)(struct PopWorkitemResponseWrapper*));

struct UpdateWorkitemResponseWrapper *update_workitem(struct ClientWrapper *client,
                                                      struct UpdateWorkitemRequestWrapper *options);

void update_workitem_async(struct ClientWrapper *client,
                           struct UpdateWorkitemRequestWrapper *options,
                           void (*callback)(struct UpdateWorkitemResponseWrapper*));

void free_update_workitem_response(struct UpdateWorkitemResponseWrapper *response);

struct DeleteWorkitemResponseWrapper *delete_workitem(struct ClientWrapper *client,
                                                      struct DeleteWorkitemRequestWrapper *options);

void delete_workitem_async(struct ClientWrapper *client,
                           struct DeleteWorkitemRequestWrapper *options,
                           void (*callback)(struct DeleteWorkitemResponseWrapper*));

void free_delete_workitem_response(struct DeleteWorkitemResponseWrapper *response);

struct ClientEventResponseWrapper *on_client_event(struct ClientWrapper *client);

struct ClientEventResponseWrapper *on_client_event_async(struct ClientWrapper *client,
                                                         ClientEventCallback event_callback);

struct ClientEventWrapper *next_client_event(const char *clientid);

struct OffClientEventResponseWrapper *off_client_event(const char *eventid);

void free_off_event_response(struct OffClientEventResponseWrapper *response);

void free_event_response(struct ClientEventResponseWrapper *response);

void free_client_event(struct ClientEventWrapper *response);

struct RpcResponseWrapper *rpc(struct ClientWrapper *client,
                               struct QueueMessageRequestWrapper *options,
                               int32_t timeout);

void rpc_async(struct ClientWrapper *client,
               struct QueueMessageRequestWrapper *options,
               RpcResponseCallback response_callback,
               int32_t timeout);

void free_rpc_response(struct RpcResponseWrapper *response);

struct InvokeOpenRPAResponseWrapper *invoke_openrpa(struct ClientWrapper *client,
                                                    struct InvokeOpenRPARequestWrapper *options,
                                                    int32_t timeout);

void free_invoke_openrpa_response(struct InvokeOpenRPAResponseWrapper *response);
