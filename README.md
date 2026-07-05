# CedarPolicy

CedarPolicy is an Elixir wrapper for the [cedar-policy](https://crates.io/crates/cedar-policy) rust library using [rustler](https://github.com/rusterlium/rustler).

It has all the basic features provided by it's rust package including support for JSON, templates, context etc.

## Installation

The package can be installed by adding `cedar_policy` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:cedar_policy, "~> 0.0.1"}
  ]
end
```

The docs can be found at <https://hexdocs.pm/cedar_policy>.

## Todo before initial release

- Use rustler precompiled to skip rust compilation during installation for end-user
- Clean up & structure tests in better way
- Improve error details being passed down from rust

---

#### Use this during development to watch docs in browser

```sh
find lib/ -name '*.ex' | entr -r mix docs | erl -S httpd serve doc/
```
