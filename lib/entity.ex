defmodule CedarPolicy.Entity do
  @moduledoc """
  Represents an entity in the Cedar policy system which consists of a unique identifier, attributes, parent entities, and tags.
  """
  defstruct [:id, :attrs, :parents, :tags]

  alias CedarPolicy.EntityUid
  alias CedarPolicy.Record

  @type t() :: %__MODULE__{id: EntityUid.t(), attrs: Record.t(), parents: list(EntityUid.t()), tags: Record.t()}

  @doc """
  Creates a new `CedarPolicy.Entity` struct.

  ## Parameters
    - `id`: The unique identifier for the entity, which must be a `CedarPolicy.EntityUid` struct.
    - `attrs`: A list of attributes associated with the entity (optional).
    - `parents`: A list of parent entities represented by their unique identifiers (optional).
    - `tags`: A list of tags associated with the entity (optional).

  ## Examples

      iex> Entity.new(EntityUid.new(:user, :123))
      %Entity{id: %EntityUid{type_name: "user", id: "123"}, attrs: [], parents: [], tags: []}

      iex> Entity.new(EntityUid.new(:user, :123), [name: "Alice"], [], [active: true])
      %Entity{
        id: %EntityUid{type_name: "user", id: "123"},
        attrs: [name: "Alice"],
        tags: [active: true],
        parents: []
      }
  """
  @spec new(id :: EntityUid.t(), attrs :: Record.t(), parents :: list(EntityUid.t()), tags :: Record.t()) :: t()

  def new(id, attrs \\ [], parents \\ [], tags \\ [])

  def new(id, attrs, parents, tags)
      when is_struct(id, EntityUid) and
             is_list(parents) and
             is_list(attrs) and
             is_list(tags) do
    %__MODULE__{
      id: id,
      attrs: attrs,
      parents: parents,
      tags: tags
    }
  end
end
