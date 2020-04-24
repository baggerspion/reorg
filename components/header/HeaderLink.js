import { useRouter } from 'next/router';

const HeaderLink = (props) => {
    const router = useRouter()
    const className = router.pathname === props.href ? "nav-item active" : "nav-item";

    return (
        <li className={className}>
            <a className="nav-link" href={props.href}>{props.name}</a>
        </li>
    );
}

export default HeaderLink;