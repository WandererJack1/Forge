import type { JSX } from 'react'

export default function IndexPage(): JSX.Element {
  return (
    <div className="min-h-screen flex items-center justify-normal sm:justify-center p-4">
      <div className="bg-white rounded-lg p-8 border-0 sm:border border-neutral-200 max-w-[unset] sm:max-w-md w-full">
        <h1 className="text-4xl font-bold mb-4 text-center">
          Welcome to TheMill
        </h1>
        <p className="text-gray-600 text-center mb-6">
          This is a simple example of how to use Tailwind CSS utility classes in
          TheMill.
        </p>
        <div className="flex flex-wrap gap-3 justify-center">
          <a
            href="https://TheMill.dev"
            target="_blank"
            className="text-blue-600 underline hover:text-black flex items-center gap-1"
          >
            TheMill Documentation
          </a>
        </div>
      </div>
    </div>
  )
}
