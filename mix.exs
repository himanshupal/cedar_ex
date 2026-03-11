defmodule CedarPolicy.MixProject do
  use Mix.Project

  def project do
    [
      app: :cedar_policy,
      version: "0.0.0-development",
      elixir: "~> 1.19",
      start_permanent: Mix.env() == :prod,
      description: "Elixir bindings for cedar_policy rust package",
      source_url: "https://github.com/himanshupal/cedar_ex",
      deps: deps()
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.37.3", runtime: false}
    ]
  end
end
