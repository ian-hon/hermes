"use client";

import { useState } from 'react';
import styles from './login.module.css';
import Image from 'next/image';
import { BACKEND_ADDRESS } from '../constants';
import { sendPostRequest } from '../utils';
import { redirect } from 'next/navigation';

export default function loginPage() {
    const [userState, changeUserState] = useState(true);

    const [username, changeUsername] = useState('lorem');
    const [password, changePassword] = useState('ipsum');
    const [confirmPassword, changeConfirmPassword] = useState('');

    const [showPassword, toggleShowPassword] = useState(false);

    function attemptLogin(): any {
        sendPostRequest(`${BACKEND_ADDRESS}/user/login`, {
            "username": username,
            "password": password
        }, (r: any) => {
            let response = JSON.parse(r);
            if (response['Success'] != undefined) {
                localStorage.setItem("session", response['Success']);
                localStorage.setItem("username", username);
                redirect("/");
            }
        })
    }
    
    return <div id={styles.main}>
        <div id={styles.container}>
            <div id={styles.header}>
                <Image src={'/hermes_icon.png'} alt='' width={150} height={150}></Image>
                <h1>
                    hermes
                </h1>
                <h4>
                    messenger for the gods
                </h4>
            </div>
            <div id={styles.userFields}>
                <input id={styles.username} value={username} onChange={(e) => changeUsername(e.target.value)} placeholder="username"/>
                <div>
                    <input type={ showPassword ? "text" : "password" } id={styles.password} value={password} onChange={(e) => changePassword(e.target.value)} placeholder="password"/>
                    <Image onClick={() => { toggleShowPassword(!showPassword) }} src={ showPassword ? "/eye.svg" : "/eye-slash.svg"} alt='' height='50' width='50'/>
                </div>
                <div>
                    <input type="password" aria-label={ userState ? 'log' : 'sign' } id={styles.confirmPassword} value={confirmPassword} onChange={(e) => changeConfirmPassword(e.target.value)} placeholder="confirm your password"/>
                </div>
                <div id={styles.actions}>
                    <h2 onClick={attemptLogin}>
                        { userState ? 'log in' : 'sign up' }
                    </h2>
                    <h4 onClick={() => { changeUserState(!userState); }}>
                        { userState ? 'no account? sign up' : 'have an account? log in' }
                    </h4>
                </div>
            </div>
        </div>
    </div>
}