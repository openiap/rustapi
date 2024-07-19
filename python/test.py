from openiap import Client, ClientError
import time

# Main function
# Example usage
if __name__ == "__main__":
    try:
        client = Client('http://localhost:50051')
        signin_result = client.signin()
        print(signin_result)
    except ClientError as e:
        print(f"An error occurred: {e}")
