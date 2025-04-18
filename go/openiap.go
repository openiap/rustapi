package openiap

/*
#cgo linux,amd64 LDFLAGS: -L./lib -lopeniap-linux-x64
#cgo linux,arm64 LDFLAGS: -L./lib -lopeniap-linux-arm64
#cgo darwin,amd64 LDFLAGS: -L./lib -lopeniap-macos-x64
#cgo darwin,arm64 LDFLAGS: -L./lib -lopeniap-macos-arm64
#cgo windows,amd64 LDFLAGS: -L./lib -lopeniap-windows-x64
#cgo windows,386 LDFLAGS: -L./lib -lopeniap-windows-i686
#cgo CFLAGS: -I.
#include "clib_openiap.h"
#include <stdlib.h>
*/
import "C"
import (
	"errors"
	"unsafe"
)

// Client represents a connection to the OpenIAP platform
type Client struct {
	ptr unsafe.Pointer
}

// NewClient creates a new OpenIAP client
func NewClient() (*Client, error) {
	clientPtr := C.create_client()
	clientWrapper := (*C.struct_ClientWrapper)(clientPtr)

	if clientWrapper == nil || !clientWrapper.success {
		errorMsg := C.GoString(clientWrapper.error)
		return nil, errors.New("failed to create OpenIAP client: " + errorMsg)
	}
	client := &Client{ptr: unsafe.Pointer(clientWrapper)}
	client.SetAgentName("go") // Set default agent name
	return client, nil
}

// SetAgentName sets the agent name for the client
func (c *Client) SetAgentName(name string) {
	cName := C.CString(name)
	defer C.free(unsafe.Pointer(cName))
	C.client_set_agent_name((*C.struct_ClientWrapper)(c.ptr), cName)
}
// SetDefaultTimeout sets the agent's default timeout for all server commands
func (c *Client) SetDefaultTimeout(timeout int) {
	C.client_set_default_timeout((*C.struct_ClientWrapper)(c.ptr), timeout)
}

// EnableTracing enables tracing for debugging
func EnableTracing(rustLog, tracing string) {
	cRustLog := C.CString(rustLog)
	cTracing := C.CString(tracing)
	defer C.free(unsafe.Pointer(cRustLog))
	defer C.free(unsafe.Pointer(cTracing))

	C.enable_tracing(cRustLog, cTracing)
}

// Connect attempts to connect the client to the specified URL
func (c *Client) Connect(url string) error {
	cUrl := C.CString(url)
	defer C.free(unsafe.Pointer(cUrl))

	responsePtr := C.client_connect((*C.struct_ClientWrapper)(c.ptr), cUrl) // Fix: Ensure correct struct type
	if responsePtr == nil {
		return errors.New("connection failed")
	}

	response := (*C.struct_ConnectResponseWrapper)(responsePtr)
	if !response.success {
		errorMsg := C.GoString(response.error)
		return errors.New("connection error: " + errorMsg)
	}
	return nil
}

// Free releases the client instance
func (c *Client) Free() {
	if c.ptr != nil {
		C.free_client((*C.struct_ClientWrapper)(c.ptr)) // Fix: Ensure correct struct type
		c.ptr = nil
	}
}

// QueryRequest represents parameters for querying data
type QueryRequest struct {
	Collection string
	Query      string
	Projection string
	OrderBy    string
	Skip       int
	Top        int
	QueryAs    string
	Explain    bool
}

// Query executes a query against the OpenIAP platform
func (c *Client) Query(req QueryRequest) (string, error) {
	cCollection := C.CString(req.Collection)
	cQuery := C.CString(req.Query)
	cProj := C.CString(req.Projection)
	cOrderBy := C.CString(req.OrderBy)
	cQueryAs := C.CString(req.QueryAs)
	defer C.free(unsafe.Pointer(cCollection))
	defer C.free(unsafe.Pointer(cQuery))
	defer C.free(unsafe.Pointer(cProj))
	defer C.free(unsafe.Pointer(cOrderBy))
	defer C.free(unsafe.Pointer(cQueryAs))

	queryReq := &C.struct_QueryRequestWrapper{
		collectionname: cCollection,
		query:          cQuery,
		projection:     cProj,
		orderby:        cOrderBy,
		queryas:        cQueryAs,
		explain:        C._Bool(req.Explain),
		skip:           C.int(req.Skip),
		top:            C.int(req.Top),
	}

	responsePtr := C.query((*C.struct_ClientWrapper)(c.ptr), queryReq)
	if responsePtr == nil {
		return "", errors.New("query failed")
	}

	response := (*C.struct_QueryResponseWrapper)(responsePtr)
	defer C.free_query_response(responsePtr)

	if !response.success {
		return "", errors.New(C.GoString(response.error))
	}

	return C.GoString(response.results), nil
}

// InsertOneRequest represents parameters for inserting a document
type InsertOneRequest struct {
	Collection string
	Item       string
	W          int
	J          bool
}

// InsertOne inserts a single document into the specified collection
func (c *Client) InsertOne(req InsertOneRequest) (string, error) {
	cCollection := C.CString(req.Collection)
	cItem := C.CString(req.Item)
	defer C.free(unsafe.Pointer(cCollection))
	defer C.free(unsafe.Pointer(cItem))

	insertReq := &C.struct_InsertOneRequestWrapper{
		collectionname: cCollection,
		item:           cItem,
		w:              C.int(req.W),
		j:              C._Bool(req.J),
	}

	responsePtr := C.insert_one((*C.struct_ClientWrapper)(c.ptr), insertReq)
	if responsePtr == nil {
		return "", errors.New("insert failed")
	}

	response := (*C.struct_InsertOneResponseWrapper)(responsePtr)
	defer C.free_insert_one_response(responsePtr)

	if !response.success {
		return "", errors.New(C.GoString(response.error))
	}

	return C.GoString(response.result), nil
}

// WatchRequest represents parameters for watching a collection
type WatchRequest struct {
	Collection string
	Paths      string
}

// WatchResponse represents a watch event from the server
type WatchResponse struct {
	ID        string
	Operation string
	Document  string
}

// Watch starts watching a collection for changes
func (c *Client) Watch(req WatchRequest) (string, error) {
	cCollection := C.CString(req.Collection)
	cPaths := C.CString(req.Paths)
	defer C.free(unsafe.Pointer(cCollection))
	defer C.free(unsafe.Pointer(cPaths))

	watchReq := &C.struct_WatchRequestWrapper{
		collectionname: cCollection,
		paths:          cPaths,
	}

	responsePtr := C.watch((*C.struct_ClientWrapper)(c.ptr), watchReq)
	if responsePtr == nil {
		return "", errors.New("watch failed")
	}

	response := (*C.struct_WatchResponseWrapper)(responsePtr)
	defer C.free_watch_response(responsePtr)

	if !response.success {
		return "", errors.New(C.GoString(response.error))
	}

	return C.GoString(response.watchid), nil
}

// NextWatchEvent gets the next watch event for a given watch ID
func (c *Client) NextWatchEvent(watchID string) (*WatchResponse, error) {
	cWatchID := C.CString(watchID)
	defer C.free(unsafe.Pointer(cWatchID))

	responsePtr := C.next_watch_event(cWatchID)
	if responsePtr == nil {
		return nil, nil // No event available
	}

	response := (*C.struct_WatchEventWrapper)(responsePtr)
	defer C.free_watch_event(responsePtr)

	if response.id == nil {
		return nil, nil // No event available
	}

	return &WatchResponse{
		ID:        C.GoString(response.id),
		Operation: C.GoString(response.operation),
		Document:  C.GoString(response.document),
	}, nil
}

// SetF64ObservableGauge sets a float64 observable gauge
func SetF64ObservableGauge(name string, value float64, description string) {
	cName := C.CString(name)
	cDesc := C.CString(description)
	defer C.free(unsafe.Pointer(cName))
	defer C.free(unsafe.Pointer(cDesc))

	C.set_f64_observable_gauge(cName, C.double(value), cDesc)
}

// SetU64ObservableGauge sets a uint64 observable gauge
func SetU64ObservableGauge(name string, value uint64, description string) {
	cName := C.CString(name)
	cDesc := C.CString(description)
	defer C.free(unsafe.Pointer(cName))
	defer C.free(unsafe.Pointer(cDesc))

	C.set_u64_observable_gauge(cName, C.uint64_t(value), cDesc)
}

// SetI64ObservableGauge sets an int64 observable gauge
func SetI64ObservableGauge(name string, value int64, description string) {
	cName := C.CString(name)
	cDesc := C.CString(description)
	defer C.free(unsafe.Pointer(cName))
	defer C.free(unsafe.Pointer(cDesc))

	C.set_i64_observable_gauge(cName, C.int64_t(value), cDesc)
}

// DisableObservableGauge disables an observable gauge
func DisableObservableGauge(name string) {
	cName := C.CString(name)
	defer C.free(unsafe.Pointer(cName))

	C.disable_observable_gauge(cName)
}
