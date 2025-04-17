export class Client {
    lib: koffi.IKoffiLib;
    connected: boolean;
    client: any;
    formatBytes(bytes: any, decimals?: number): string;
    free(): void;
    enable_tracing(rust_log?: string, tracing?: string): void;
    informing: boolean;
    disable_tracing(): void;
    error(...args: any[]): void;
    info(...args: any[]): void;
    warn(...args: any[]): void;
    debug(...args: any[]): void;
    trace(...args: any[]): void;
    set_f64_observable_gauge(name: any, value: any, description: any): void;
    set_u64_observable_gauge(name: any, value: any, description: any): void;
    set_i64_observable_gauge(name: any, value: any, description: any): void;
    disable_observable_gauge(name: any): void;
    set_agent_name(name: any): void;
    set_agent_version(version: any): void;
    stringify(obj: any): string;
    connect(url?: string): Promise<any>;
    connect_async(url?: string): any;
    disconnect(): void;
    signin({ username, password, jwt, agent, version, longtoken, validateonly, ping }?: {
        username?: string;
        password?: string;
        jwt?: string;
        agent?: string;
        version?: string;
        longtoken?: boolean;
        validateonly?: boolean;
        ping?: boolean;
    }): {
        success: any;
        jwt: any;
        error: any;
    };
    signin_async({ username, password, jwt, agent, version, longtoken, validateonly, ping }?: {
        username?: string;
        password?: string;
        jwt?: string;
        agent?: string;
        version?: string;
        longtoken?: boolean;
        validateonly?: boolean;
        ping?: boolean;
    }): any;
    list_collections(includehist?: boolean): any;
    list_collections_async(includehist?: boolean): any;
    create_collection({ collectionname, collation, timeseries, expire_after_seconds, change_stream_pre_and_post_images, capped, max, size }: {
        collectionname: any;
        collation?: any;
        timeseries?: any;
        expire_after_seconds?: number;
        change_stream_pre_and_post_images?: boolean;
        capped?: boolean;
        max?: number;
        size?: number;
    }): any;
    create_collection_async({ collectionname, collation, timeseries, expire_after_seconds, change_stream_pre_and_post_images, capped, max, size }: {
        collectionname: any;
        collation?: any;
        timeseries?: any;
        expire_after_seconds?: number;
        change_stream_pre_and_post_images?: boolean;
        capped?: boolean;
        max?: number;
        size?: number;
    }): any;
    drop_collection(collectionname: any): void;
    drop_collection_async(collectionname: any): any;
    get_indexes(collectionname: any): any;
    get_indexes_async(collectionname: any): any;
    create_index({ collectionname, index, options, name }: {
        collectionname: any;
        index: any;
        options?: {};
        name?: string;
    }): any;
    create_index_async({ collectionname, index, options, name }: {
        collectionname: any;
        index: any;
        options?: {};
        name?: string;
    }): any;
    drop_index(collectionname: any, indexname: any): void;
    drop_index_async(collectionname: any, indexname: any): any;
    custom_command({ command, id, name, data }: {
        command: any;
        id?: string;
        name?: string;
        data?: {};
    }): any;
    custom_command_async({ command, id, name, data }: {
        command: any;
        id?: string;
        name?: string;
        data?: {};
    }): any;
    query({ collectionname, query, projection, orderby, skip, top, queryas, explain }: {
        collectionname: any;
        query: any;
        projection?: {};
        orderby?: string;
        skip?: number;
        top?: number;
        queryas?: string;
        explain?: boolean;
    }): any;
    query_async({ collectionname, query, projection, orderby, skip, top, queryas, explain }: {
        collectionname: any;
        query: any;
        projection?: {};
        orderby?: string;
        skip?: number;
        top?: number;
        queryas?: string;
        explain?: boolean;
    }): any;
    aggregate({ collectionname, aggregates, queryas, hint, explain }: {
        collectionname: any;
        aggregates?: any[];
        queryas?: string;
        hint?: string;
        explain?: boolean;
    }): any;
    aggregate_async({ collectionname, aggregates, queryas, hint, explain }: {
        collectionname: any;
        aggregates?: any[];
        queryas?: string;
        hint?: string;
        explain?: boolean;
    }): any;
    count({ collectionname, query, queryas, explain }: {
        collectionname: any;
        query?: {};
        queryas?: string;
        explain?: boolean;
    }): any;
    count_async({ collectionname, query, queryas, explain }: {
        collectionname: any;
        query?: {};
        queryas?: string;
        explain?: boolean;
    }): any;
    distinct({ collectionname, field, query, queryas, explain }: {
        collectionname: any;
        field: any;
        query?: {};
        queryas?: string;
        explain?: boolean;
    }): any[];
    distinct_async({ collectionname, field, query, queryas, explain }: {
        collectionname: any;
        field: any;
        query?: {};
        queryas?: string;
        explain?: boolean;
    }): any;
    insert_one({ collectionname, item, w, j }: {
        collectionname: any;
        item: any;
        w?: number;
        j?: boolean;
    }): any;
    insert_one_async({ collectionname, item, w, j }: {
        collectionname: any;
        item: any;
        w?: number;
        j?: boolean;
    }): any;
    insert_many({ collectionname, items, w, j, skipresults }: {
        collectionname: any;
        items?: any[];
        w?: number;
        j?: boolean;
        skipresults?: boolean;
    }): any;
    insert_many_async({ collectionname, items, w, j, skipresults }: {
        collectionname: any;
        items?: any[];
        w?: number;
        j?: boolean;
        skipresults?: boolean;
    }): any;
    update_one({ collectionname, item, w, j }: {
        collectionname: any;
        item: any;
        w?: number;
        j?: boolean;
    }): any;
    update_one_async({ collectionname, item, w, j }: {
        collectionname: any;
        item: any;
        w?: number;
        j?: boolean;
    }): any;
    insert_or_update_one({ collectionname, item, uniqeness, w, j }: {
        collectionname: any;
        item: any;
        uniqeness?: string;
        w?: number;
        j?: boolean;
    }): any;
    insert_or_update_one_async({ collectionname, item, uniqeness, w, j }: {
        collectionname: any;
        item: any;
        uniqeness?: string;
        w?: number;
        j?: boolean;
    }): any;
    delete_one({ collectionname, id, recursive }: {
        collectionname: any;
        id: any;
        recursive?: boolean;
    }): any;
    delete_one_async({ collectionname, id, recursive }: {
        collectionname: any;
        id: any;
        recursive?: boolean;
    }): any;
    delete_many({ collectionname, query, recursive }: {
        collectionname: any;
        query: any;
        recursive?: boolean;
    }): any;
    delete_many_async({ collectionname, query, recursive }: {
        collectionname: any;
        query: any;
        recursive?: boolean;
    }): any;
    download({ collectionname, id, folder, filename }: {
        collectionname: any;
        id: any;
        folder?: string;
        filename?: string;
    }): any;
    download_async({ collectionname, id, folder, filename }: {
        collectionname: any;
        id: any;
        folder?: string;
        filename?: string;
    }): any;
    upload({ filepath, filename, mimetype, metadata, collectionname }: {
        filepath: any;
        filename: any;
        mimetype?: string;
        metadata?: {};
        collectionname?: string;
    }): any;
    upload_async({ filepath, filename, mimetype, metadata, collectionname }: {
        filepath: any;
        filename: any;
        mimetype?: string;
        metadata?: {};
        collectionname?: string;
    }): any;
    push_workitem({ wiq, wiqid, name, payload, nextrun, success_wiqid, failed_wiqid, success_wiq, failed_wiq, priority, files }: {
        wiq?: string;
        wiqid?: string;
        name: any;
        payload?: {};
        nextrun?: number;
        success_wiqid?: string;
        failed_wiqid?: string;
        success_wiq?: string;
        failed_wiq?: string;
        priority?: number;
        files?: any[];
    }): any;
    push_workitem_async({ wiq, wiqid, name, payload, nextrun, success_wiqid, failed_wiqid, success_wiq, failed_wiq, priority, files }: {
        wiq?: string;
        wiqid?: string;
        name: any;
        payload?: {};
        nextrun?: number;
        success_wiqid?: string;
        failed_wiqid?: string;
        success_wiq?: string;
        failed_wiq?: string;
        priority?: number;
        files?: any[];
    }): any;
    pop_workitem({ wiq, wiqid, downloadfolder }: {
        wiq?: string;
        wiqid?: string;
        downloadfolder?: string;
    }): any;
    callbackid: number;
    callbacks: {};
    pop_workitem_async_callback(responsePtr: any): void;
    pop_workitem_async({ wiq, wiqid, downloadfolder }: {
        wiq?: string;
        wiqid?: string;
        downloadfolder?: string;
    }): any;
    update_workitem({ workitem, ignoremaxretries, files }: {
        workitem: any;
        ignoremaxretries?: boolean;
        files?: any[];
    }): any;
    update_workitem_async({ workitem, ignoremaxretries, files }: {
        workitem: any;
        ignoremaxretries?: boolean;
        files?: any[];
    }): any;
    delete_workitem(id: any): void;
    delete_workitem_async(id: any): any;
    watches: {};
    next_watch_interval: number;
    /**
     * @typedef {object} WatchOptions
     * @property {string}   collectionname  The name of the collection to watch.
     * @property {string[]} [paths=[]]      Specific document paths to filter on.
     */
    /**
     * @typedef {object} WatchEvent
     * @property {string} id          The document’s unique ID.
     * @property {string} operation   The type of change ("insert", "update", "delete", etc.).
     * @property {any}    document    The JSON‑parsed document payload.
     */
    /**
     * @callback WatchCallback
     * @param {WatchEvent} event         The watch event.
     * @param {number}     event_counter How many events have fired so far.
     * @returns {void}   Throw or log inside if you need to handle errors.
     */
    /**
     * Start watching a collection.
     *
     * @param {WatchOptions} options
     * @param {WatchCallback} callback
     * @returns {string}  The watch ID you can later cancel.
     */
    watch({ collectionname, paths }: {
        /**
         * The name of the collection to watch.
         */
        collectionname: string;
        /**
         * Specific document paths to filter on.
         */
        paths?: string[];
    }, callback: (event: {
        /**
         * The document’s unique ID.
         */
        id: string;
        /**
         * The type of change ("insert", "update", "delete", etc.).
         */
        operation: string;
        /**
         * The JSON‑parsed document payload.
         */
        document: any;
    }, event_counter: number) => void): string;
    clientevents: {};
    on_client_event(callback: any): any;
    off_client_event(eventid: any): void;
    event_refs: {};
    uniqeid(): string;
    watch_async({ collectionname, paths }: {
        collectionname: any;
        paths?: any[];
    }, callback: any): any;
    watch_async_async({ collectionname, paths }: {
        collectionname: any;
        paths?: any[];
    }, callback: any): any;
    unwatch(watchid: any): void;
    queues: {};
    next_queue_interval: number;
    /**
     * @typedef {object} QueueEvent
     * @property {string} queuename       The name of the queue that produced this event.
     * @property {string} correlation_id   The ID you can use to correlate replies.
     * @property {string} replyto          The queue name to reply to (if any).
     * @property {string} routingkey       The routing key from the broker.
     * @property {string} exchangename     The exchange this event came from.
     * @property {any}    data             The payload (already JSON‑parsed if valid).
     * @property {string} [jwt]            A stripped‑out JWT, if one was present.
     * @property {any}    [user]           A stripped‑out user object, if one was present.
     */
    /**
     * @callback QueueCallback
     * @param {QueueEvent} event   The fully‑typed event object.
     * @returns {Promise<any>|any}  If you return something and `event.replyto` is set, it'll be sent.
     */
    /**
     * Register a queue and start pumping events into your callback.
     *
     * @param {{ queuename: string }} options            Options bag.
     * @param {QueueCallback}       callback            Called for each event.
     * @returns {string}            The (possibly‑rewritten) queue name.
     */
    register_queue({ queuename }: {
        queuename: string;
    }, callback: (event: {
        /**
         * The name of the queue that produced this event.
         */
        queuename: string;
        /**
         * The ID you can use to correlate replies.
         */
        correlation_id: string;
        /**
         * The queue name to reply to (if any).
         */
        replyto: string;
        /**
         * The routing key from the broker.
         */
        routingkey: string;
        /**
         * The exchange this event came from.
         */
        exchangename: string;
        /**
         * The payload (already JSON‑parsed if valid).
         */
        data: any;
        /**
         * A stripped‑out JWT, if one was present.
         */
        jwt?: string;
        /**
         * A stripped‑out user object, if one was present.
         */
        user?: any;
    }) => Promise<any> | any): string;
    /**
     * @typedef {object} RegisterExchangeOptions
     * @property {string} exchangename    The exchange to bind.
     * @property {string} [algorithm="fanout"]  Exchange algorithm (e.g. "fanout", "direct", "topic").
     * @property {string} [routingkey=""]      Default routing key to use.
     * @property {boolean} [addqueue=true]     Whether to auto‑create a queue for this exchange.
     */
    /**
     * @typedef {object} ExchangeEvent
     * @property {string} queuename       The name of the queue that produced this event.
     * @property {string} correlation_id   Correlation ID for replies.
     * @property {string} replyto          Queue name to send replies to.
     * @property {string} routingkey       The routing key of this message.
     * @property {string} exchangename     The exchange name that emitted this event.
     * @property {any}    data             The raw payload.
     */
    /**
     * @callback ExchangeCallback
     * @param {ExchangeEvent} event  The decoded exchange event.
     * @returns {void|Promise<void>}  If the handler is async, return a Promise.
     */
    /**
     * Register an exchange (and auto‑create a queue if requested).
     *
     * @param {RegisterExchangeOptions} options
     * @param {ExchangeCallback}       callback
     * @returns {string}  The (possibly‑rewritten) queue name for this exchange.
     */
    register_exchange({ exchangename, algorithm, routingkey, addqueue }: {
        /**
         * The exchange to bind.
         */
        exchangename: string;
        /**
         * Exchange algorithm (e.g. "fanout", "direct", "topic").
         */
        algorithm?: string;
        /**
         * Default routing key to use.
         */
        routingkey?: string;
        /**
         * Whether to auto‑create a queue for this exchange.
         */
        addqueue?: boolean;
    }, callback: (event: {
        /**
         * The name of the queue that produced this event.
         */
        queuename: string;
        /**
         * Correlation ID for replies.
         */
        correlation_id: string;
        /**
         * Queue name to send replies to.
         */
        replyto: string;
        /**
         * The routing key of this message.
         */
        routingkey: string;
        /**
         * The exchange name that emitted this event.
         */
        exchangename: string;
        /**
         * The raw payload.
         */
        data: any;
    }) => void | Promise<void>): string;
    rpc({ queuename, data, striptoken }: {
        queuename: any;
        data?: {};
        striptoken?: boolean;
    }): any;
    rpc_async({ queuename, data, striptoken }: {
        queuename: any;
        data?: {};
        striptoken?: boolean;
    }): any;
    unregister_queue(queuename: any): void;
    queue_message({ queuename, data, replyto, exchangename, correlation_id, routingkey, striptoken, expiration }: {
        queuename: any;
        data?: {};
        replyto?: string;
        exchangename?: string;
        correlation_id?: string;
        routingkey?: string;
        striptoken?: boolean;
        expiration?: number;
    }): void;
}
export class ClientError extends Error {
    constructor(message: any);
}
export class LibraryLoadError extends ClientError {
}
export class ClientCreationError extends ClientError {
}
import koffi = require("koffi");
//# sourceMappingURL=lib.d.ts.map