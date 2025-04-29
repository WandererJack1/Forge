// src/routes/pokemons/[pokemon].tsx
import type { JSX } from 'react'
import type { TheMillRouteProps } from 'TheMill'
import { Link } from 'TheMill'

import PokemonView from '../../components/PokemonView'

interface Pokemon {
  id: number
  name: string
  weight: number
  height: number
}

export default function PokemonPage({
  isLoading,
  data,
}: TheMillRouteProps<Pokemon>): JSX.Element {
  return (
    <div>
      <Link href="/">Back</Link>

      {isLoading && (
        <>
          <title>Pokemon: loading...</title>
          <div>Loading...</div>
        </>
      )}

      {data?.id && (
        <>
          <title>{`Pokemon: ${data.name}`}</title>
          <PokemonView pokemon={data} />
        </>
      )}
    </div>
  )
}
