defmodule CedarPolicy.Native do
  use Rustler, otp_app: :cedar_policy, crate: "cedar_ex"

  alias CedarPolicy.RestrictedExpression
  alias CedarPolicy.EntityUid
  alias CedarPolicy.Entity

  @spec new() :: pid()
  def new(), do: not_loaded()

  @spec add_policy(
          state :: pid(),
          policy :: String.t(),
          id :: String.t() | nil
        ) :: pid()

  def add_policy(_state, _policy, _id \\ nil)
  def add_policy(_state, _policy, _id), do: not_loaded()

  @spec add_entities(
          state :: pid(),
          entities :: list(Entity.t()),
          schema :: String.t() | nil
        ) :: pid()

  def add_entities(_state, _entities, _schema \\ nil)
  def add_entities(_state, _entities, _schema), do: not_loaded()

  @spec validate(state :: pid(), schema :: String.t()) :: boolean()
  def validate(_state, _schema), do: not_loaded()

  @spec get_lang_version() :: CedarPolicy.Version.t()
  def get_lang_version(), do: not_loaded()

  @spec get_sdk_version() :: CedarPolicy.Version.t()
  def get_sdk_version(), do: not_loaded()

  @spec verify(
          state :: pid(),
          principal :: EntityUid.t(),
          action :: EntityUid.t(),
          resource :: EntityUid.t(),
          context :: list({String.t(), RestrictedExpression.t()}),
          schema :: String.t() | nil
        ) :: boolean()
  def verify(_state, _p, _a, _r, _c, _s \\ nil)
  def verify(_state, _p, _a, _r, _c, _s), do: not_loaded()

  defp not_loaded, do: :erlang.nif_error(:nif_not_loaded)
end
