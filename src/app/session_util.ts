import { redirect } from "next/navigation";

export function checkSession(status: string) {
    // redirect to login page if is SessionIDNoExist, SessionIDInvalid or SessionIDExpired
    if ([
        "SessionIDNoExist",
        "SessionIDInvalid",
        "SessionIDExpired"
    ].includes(status)) {
        redirect("login");
    }
}
