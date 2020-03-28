import Link from 'next/link';

export default function Index() {
  return (
    <div>
      <Link href="/about">
        <a>About Page</a>
      </Link>
      <h1>Hello World!</h1>
    </div>
  );
}
