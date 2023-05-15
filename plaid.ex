defmodule PlaidClient do
  @base_url "https://api.plaid.com"

  def exchange_public_token(public_token, client_id, secret) do
    params = %{
      "client_id" => client_id,
      "secret" => secret,
      "public_token" => public_token
    }

    response = HTTPoison.post(
      "#{@base_url}/item/public_token/exchange",
      Poison.encode!(%{"Content-Type" => "application/json"}, params)
    )

    case response.status_code do
      200 ->
        {:ok, body} = response.body
        {:ok, %{"access_token" => access_token, "item_id" => item_id}} = Poison.decode!(body)
        {:ok, access_token, item_id}

      _ ->
        {:error, %{reason: "Plaid token exchange failed", body: response.body}}
    end
  end
end

defmodule MyApp do
  def run_demo do
    public_token = "your_public_token"
    client_id = "your_client_id"
    secret = "your_secret"

    case PlaidClient.exchange_public_token(public_token, client_id, secret) do
      {:ok, access_token, item_id} ->
        IO.puts("Access Token: #{access_token}")
        IO.puts("Item ID: #{item_id}")
        # Perform further operations with the access token
        # such as retrieving account details, transactions, etc.

      {:error, %{reason: error_reason, body: error_body}} ->
        IO.puts("Plaid token exchange error: #{error_reason}")
        IO.inspect(error_body)
    end
  end
end

MyApp.run_demo()
