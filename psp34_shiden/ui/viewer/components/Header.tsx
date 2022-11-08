import Link from 'next/link';
import { useEffect, useState } from 'react';
import { useTheme } from 'next-themes';

const Header = (): JSX.Element => {

  const { theme, setTheme } = useTheme();

  const [mounted, setMounted] = useState(false);
  useEffect(() => setMounted(true), []);

  return (
    <header className="mx-auto max-w-6xl w-11/12">
      <div className="flex flex-wrap justify-between items-center pt-4 pb-5 p-2">
        <div className="w-10/12 text-left">
          <h1 className="text-3xl">
            <Link href="/">
              <a className="text-dark">PSP34 Viewer Sample</a>
            </Link>
          </h1>
        </div>

        <div className="w-2/12 text-right">
          <button
            aria-label="DarkModeToggle"
            type="button"
            onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
          >
            {mounted && (
              <>
                {theme === 'dark' ? (
                  <div className='h-8 pt-3'><img className="h-8" src="./icon_sun.svg" alt="DarkMode" /></div>
                ) : (
                  <div className='h-8 pt-3'><img className="h-7" src="./icon_moon.svg" alt="LightMode" /></div>
                )}
              </>
            )}
          </button>
        </div>

      </div>
    </header>
  );
};

export default Header;