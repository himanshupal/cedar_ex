defmodule CedarPolicy.MixProject do
  use Mix.Project

  @version "0.0.0-development"

  def project do
    [
      app: :cedar_policy,
      version: @version,
      elixir: "~> 1.19",
      start_permanent: Mix.env() == :prod,
      description: "Elixir bindings for cedar-policy rust package",
      source_url: "https://github.com/himanshupal/cedar_ex",
      package: package(),
      deps: deps()
    ]
  end

  defp deps do
    [
      {:ex_doc, "~> 0.40.1", only: :dev, runtime: false},
      {:rustler, "~> 0.37.3", runtime: false}
    ]
  end

  defp package() do
    [
      licenses: ["Unlicense"],
      links: %{"GitHub" => "https://github.com/himanshupal/cedar_ex"}
    ]
  end
end
