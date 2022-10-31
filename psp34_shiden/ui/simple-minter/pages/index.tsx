import { NextPage } from 'next'
import dynamic from 'next/dynamic'

const Canvas = dynamic(() => import('../components/IndexCanvas'), {
  ssr: false,
  loading: () => <p>Now Loading...</p>,
})

const IndexPage: NextPage = () => {
  return (
    <main>
      <Canvas />
    </main>
  )
};

export default IndexPage;