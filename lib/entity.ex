defmodule CedarPolicy.Entity do
  defstruct [:id, :attrs, :parents, :tags]

  alias CedarPolicy.EntityUid
  alias CedarPolicy.RestrictedExpression

  @type t() :: %CedarPolicy.Entity{
          id: EntityUid.t(),
          attrs: list({String.t(), RestrictedExpression.t()}),
          parents: list(EntityUid.t()),
          tags: list({String.t(), RestrictedExpression.t()})
        }

  @spec new(
          id :: EntityUid.t(),
          attrs ::
            list({String.t(), RestrictedExpression.t()}),
          parents :: list(EntityUid.t()),
          tags :: list({String.t(), RestrictedExpression.t()})
        ) :: t()

  def new(id, attrs \\ [], parents \\ [], tags \\ [])

  def new(id, attrs, parents, tags) when is_struct(id, EntityUid) do
    %CedarPolicy.Entity{
      id: id,
      attrs: attrs,
      parents: parents,
      tags: tags
    }
  end
end
