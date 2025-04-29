import type { JSX } from 'react'
import type { TheMillRouteProps } from 'TheMill'
import { Link } from 'TheMill'
import type { MyResponse } from 'TheMill/types'

export default function IndexPage({
  data,
  isLoading,
}: TheMillRouteProps<MyResponse>): JSX.Element {
  if (isLoading) {
    return <h1>Loading...</h1>
  }

  return (
    <>
      <h1>TheMill</h1>
      <h2>{data.subtitle}</h2>
      <Link href={'/second-route'}>Routing link</Link>
    </>
  )
}
