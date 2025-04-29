// src/routes/index.tsx
import type { JSX } from 'react'
import type { TheMillRouteProps } from 'TheMill'

import PokemonLink from '../components/PokemonLink'

interface IndexProps {
  results: Array<{ name: string; url: string }>
}

export default function IndexPage({
  data,
}: TheMillRouteProps<IndexProps>): JSX.Element | null {
  if (!data?.results) return null

  return (
    <>
      <title>TheMill tutorial</title>

      <header className="header">
        <a
          href="https://crates.io/crates/TheMill"
          target="_blank"
          rel="noreferrer"
        >
          Crates
        </a>
        <a
          href="https://www.npmjs.com/package/TheMill"
          target="_blank"
          rel="noreferrer"
        >
          Npm
        </a>
      </header>
      <div className="title-wrap">
        <h1 className="title">
          TU<span>O</span>NO
        </h1>
        <div className="logo">
          <img src="rust.svg" className="rust" />
          <img src="react.svg" className="react" />
        </div>
      </div>
      <ul style={{ flexWrap: 'wrap', display: 'flex', gap: 10 }}>
        <PokemonLink name="GOAT" id={0} />

        {data.results.map((pokemon, i) => (
          <PokemonLink key={pokemon.name} name={pokemon.name} id={i + 1} />
        ))}
      </ul>
    </>
  )
}
