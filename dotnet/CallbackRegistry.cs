using System.Collections.Concurrent;
public class CallbackRegistry
{
    private ConcurrentDictionary<int, object> _callbackRegistry = new ConcurrentDictionary<int, object>();

    public int Count => _callbackRegistry.Count;

    public bool TryAddCallback<T>(int id, TaskCompletionSource<T> tcs)
    {
        return _callbackRegistry.TryAdd(id, tcs);
    }

    public bool TryAddCallback(int id, TaskCompletionSource tcs)
    {
        return _callbackRegistry.TryAdd(id, tcs);
    }

    public bool TryGetCallback<T>(int id, out TaskCompletionSource<T>? tcs)
    {
        if (_callbackRegistry.TryGetValue(id, out var obj))
        {
            if(obj is TaskCompletionSource<T> typedTcs) {
                tcs = typedTcs;
                return true;
            } else {
                Console.WriteLine("Failed to get callback for id: " + id + " is of wrong type " + obj.GetType() + " expected " + typeof(TaskCompletionSource<T>));
            }
        } else {
            Console.WriteLine("Failed to get callback for id: " + id);
        }
        tcs = null;
        return false;
    }
    public bool TryGetCallback(int id, out TaskCompletionSource? tcs)
    {
        if (_callbackRegistry.TryGetValue(id, out var obj))
        {
            if(obj is TaskCompletionSource typedTcs) {
                tcs = typedTcs;
                return true;
            } else {
                Console.WriteLine("Failed to get callback for id: " + id + " is of wrong type " + obj.GetType() + " expected " + typeof(TaskCompletionSource));
            }
        } else {
            Console.WriteLine("Failed to get callback for id: " + id);
        }
        tcs = null;
        return false;
    }

    public bool TryRemoveCallback<T>(int id, out TaskCompletionSource<T>? tcs)
    {
        if (_callbackRegistry.TryRemove(id, out var obj) && obj is TaskCompletionSource<T> typedTcs)
        {
            tcs = typedTcs;
            return true;
        }
        tcs = null;
        return false;
    }

    public bool TrySetResult<T>(int id, T result)
    {
        if (_callbackRegistry.TryRemove(id, out var obj) && obj is TaskCompletionSource<T> tcs)
        {
            tcs.SetResult(result);
            return true;
        }
        return false;
    }
    public bool TrySetResult(int id)
    {
        if (_callbackRegistry.TryRemove(id, out var obj) && obj is TaskCompletionSource tcs)
        {
            tcs.SetResult();
            return true;
        }
        return false;
    }

    public bool TrySetException<T>(int id, Exception ex)
    {
        if (_callbackRegistry.TryRemove(id, out var obj) && obj is TaskCompletionSource<T> tcs)
        {
            tcs.SetException(ex);
            return true;
        }
        return false;
    }
    public bool TrySetException(int id, Exception ex)
    {
        if (_callbackRegistry.TryRemove(id, out var obj) && obj is TaskCompletionSource tcs)
        {
            tcs.SetException(ex);
            return true;
        }
        return false;
    }
}
public class ActionObject {
    public object tcs;
    public object queuename;
    public ActionObject(object tcs, string queuename) {
        this.tcs = tcs;
        this.queuename = queuename;
    }
}
public class ActionRegistry
{
    private ConcurrentDictionary<int, object> _callbackRegistry = new ConcurrentDictionary<int, object>();
    public int Count => _callbackRegistry.Count;
    public bool TryAddCallback<T>(int id, string queuename, Action<T> tcs)
    {
        var obj = new ActionObject(tcs, queuename);
        return _callbackRegistry.TryAdd(id, obj);
    }
    public bool TryAddCallback(int id, string queuename, Action tcs)
    {
        var obj = new ActionObject(tcs, queuename);
        return _callbackRegistry.TryAdd(id, obj);
    }
    public bool TryGetCallback<T>(int id, out Action<T>? tcs)
    {
        if (_callbackRegistry.TryGetValue(id, out var _obj))
        {
            ActionObject obj = (ActionObject)_obj;
            if(obj.tcs is Action<T> typedTcs) {
                tcs = typedTcs;
                return true;
            } else {
                Console.WriteLine("Failed to get callback for id: " + id + " is of wrong type " + obj.GetType() + " expected " + typeof(Action<T>));
            }
        } else {
            Console.WriteLine("Failed to get callback for id: " + id);
        }
        tcs = null;
        return false;
    }
    public bool TryGetCallback(int id, out Action? tcs)
    {
        if (_callbackRegistry.TryGetValue(id, out var _obj))
        {
            ActionObject obj = (ActionObject)_obj;
            if(obj.tcs is Action typedTcs) {
                tcs = typedTcs;
                return true;
            } else {
                Console.WriteLine("Failed to get callback for id: " + id + " is of wrong type " + obj.GetType() + " expected " + typeof(Action));
            }
        } else {
            Console.WriteLine("Failed to get callback for id: " + id);
        }
        tcs = null;
        return false;
    }
    public bool TrySetQueueName(int id, string queuename)
    {
        if (_callbackRegistry.TryGetValue(id, out var obj))
        {
            ((ActionObject)obj).queuename = queuename;
            return true;
        }
        return false;
    }
    public bool TryRemoveCallback<T>(int id, out Action<T>? tcs)
    {
        if (_callbackRegistry.TryRemove(id, out var obj) && obj is Action<T> typedTcs)
        {
            tcs = typedTcs;
            return true;
        }
        tcs = null;
        return false;
    }
    public bool TryRemoveCallback<T>(string queuename, out Action<T>? tcs)
    {
        var id = _callbackRegistry.Where(x => (string)((ActionObject)x.Value).queuename == queuename).FirstOrDefault().Key;
        if (_callbackRegistry.TryRemove(id, out var obj) && obj is Action<T> typedTcs)
        {
            tcs = typedTcs;
            return true;
        }
        tcs = null;
        return false;
    }
}
