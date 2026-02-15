defmodule CedarPolicy.Native do
  use Rustler, otp_app: :cedar_policy, crate: "cedar"

  def new_context(), do: not_loaded()
  def increment(_context), do: not_loaded()

  defp not_loaded, do: :erlang.nif_error(:nif_not_loaded)
end
