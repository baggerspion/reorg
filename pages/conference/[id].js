import { useRouter } from 'next/router';
import Layout from '../../components/MyLayout';

export default function Conference() {
    const router = useRouter();
    
    return (
        <Layout>
            <h1>{router.query.id}</h1>
            <p>This is the conference content.</p>
        </Layout>
    );
}