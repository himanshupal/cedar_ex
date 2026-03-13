defmodule CedarPolicy.Native do
  @moduledoc false

  use Rustler, otp_app: :cedar_policy, crate: "cedar_ex"

  def new, do: not_loaded()

  def get_sdk_version, do: not_loaded()

  def get_lang_version, do: not_loaded()

  def add_policy(_store, _policy, _id \\ nil), do: not_loaded()

  def add_template(_store, _template, _id \\ nil), do: not_loaded()

  def link(_store, _template_id, _policy_id, _values), do: not_loaded()

  def add_entities(_store, _entities, _schema \\ nil), do: not_loaded()

  def validate(_store, _schema, _strict \\ nil), do: not_loaded()

  def verify(_store, _principal, _action, _resource, _context, _schema \\ nil), do: not_loaded()

  defp not_loaded, do: :erlang.nif_error(:nif_not_loaded)
end
