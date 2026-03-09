defmodule CedarPolicy.Entity do
  defstruct [:id, :attrs, :parents, :tags]

  alias CedarPolicy.EntityUid
  alias CedarPolicy.Record

  @type t() :: %CedarPolicy.Entity{
          id: EntityUid.t(),
          attrs: Record.t(),
          parents: list(EntityUid.t()),
          tags: Record.t()
        }

  @spec new(
          id :: EntityUid.t(),
          attrs :: Record.t(),
          parents :: list(EntityUid.t()),
          tags :: Record.t()
        ) :: t()

  def new(id, attrs \\ [], parents \\ [], tags \\ [])

  def new(id, attrs, parents, tags)
      when is_struct(id, EntityUid) and
             is_list(parents) and
             is_list(attrs) and
             is_list(tags) do
    %CedarPolicy.Entity{
      id: id,
      attrs: attrs,
      parents: parents,
      tags: tags
    }
  end
end
