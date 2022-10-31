import Link from 'next/link';

const Header = (): JSX.Element => {
  
  return (
    <div>
      <h1 className="my-0 p-3 mt-2 m-3 text-3xl">
        <Link href="/">
          <a className="text-dark">PSP34 Viewer Sample</a>
        </Link>
      </h1>
    </div>
  );
};

export default Header;