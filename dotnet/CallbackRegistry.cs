using System.Collections.Concurrent;
public class CallbackRegistry
{
    private ConcurrentDictionary<int, object> _callbackRegistry = new ConcurrentDictionary<int, object>();

    public int Count => _callbackRegistry.Count;

    public bool TryAddCallback<T>(int id, TaskCompletionSource<T> tcs)
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

    public bool TrySetException<T>(int id, Exception ex)
    {
        if (_callbackRegistry.TryRemove(id, out var obj) && obj is TaskCompletionSource<T> tcs)
        {
            tcs.SetException(ex);
            return true;
        }
        return false;
    }
}
