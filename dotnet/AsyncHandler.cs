using System.Text.Json;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using static OpenIAP.Client;

public static class AsyncContinuationHelper
{
    public static Task<TResult> ProcessResponseAsync<TResponse, TResult>(
        Task<string> task,  // The task with the raw JSON response
        Func<string, TResponse> responseParser,  // Function that parses JSON to response type
        Func<TResponse, TResult> resultMapper  // Function that maps the parsed response to desired result type
    )
    {
        return task.ContinueWith(task =>
        {
            if (task.IsFaulted)
            {
                throw task.Exception ?? new Exception("Unknown task failure");
            }

            string jsonString = task.Result;

            // Step 1: Parse the response JSON into a response object
            TResponse response = responseParser(jsonString);

            // Step 2: Map the parsed response into the desired result type
            return resultMapper(response);
        });
    }
}





// public class AsyncHandler
// {
//     public static Task<T> HandleAsyncCallback<T>(Action<IntPtr, QueryCallback> action)
//     {
//         var tcs = new TaskCompletionSource<string>();

//         void Callback(IntPtr responsePtr)
//         {
//             try
//             {
//                 if (responsePtr == IntPtr.Zero)
//                 {
//                     tcs.SetException(new ClientError("Callback got null response"));
//                     return;
//                 }

//                 var response = Marshal.PtrToStructure<ListCollectionsResponseWrapper>(responsePtr);
//                 string results = Marshal.PtrToStringAnsi(response.results) ?? string.Empty;
//                 string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
//                 bool success = response.success;
//                 free_query_response(responsePtr);

//                 if (!success)
//                 {
//                     tcs.SetException(new ClientError(error));
//                 }
//                 else
//                 {
//                     tcs.SetResult(results);
//                 }
//             }
//             catch (Exception ex)
//             {
//                 tcs.SetException(ex);
//             }
//         }

//         var callbackDelegate = new QueryCallback(Callback);
//         action(IntPtr.Zero, callbackDelegate);

//         return tcs.Task.ContinueWith(task =>
//         {
//             if (task.IsFaulted)
//             {
//                 throw task.Exception!;
//             }

//             string jsonString = task.Result;

//             if (typeof(T) == typeof(string))
//             {
//                 return (T)(object)jsonString;
//             }
//             else
//             {
//                 try
//                 {
//                     // Attempt to deserialize JSON into the requested type T.
//                     T result = JsonSerializer.Deserialize<T>(jsonString)!;
//                     return result;
//                 }
//                 catch (JsonException ex)
//                 {
//                     throw new InvalidOperationException($"Failed to deserialize JSON into type {typeof(T).Name}.", ex);
//                 }
//             }
//         });
//     }
// }
