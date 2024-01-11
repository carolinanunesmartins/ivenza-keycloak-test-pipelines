<#macro passwordInput id placeholder="Wachtwoord">
    <div class="relative w-full rounded-md">
        <div class="flex flex-row gap-1">
            <label class="relative contents" aria-label="${id}">
                <#--  use autocomplete="new-password" so a password is not prefilled by the browser. If you let it prefill, the browser sometimes redirects you to the logged-in keycloak account page. See https://developer.mozilla.org/en-US/docs/Web/Security/Securing_your_site/Turning_off_form_autocompletion  -->
                <input 
                    type="password" 
                    id="${id}"
                    name="${id}" 
                    placeholder="" 
                    class="pl-5 pt-2 border border-brand-gray-200 placeholder-brand-gray-600 focus:ring-brand-gray-500 focus:border-brand-gray-500 text-brand-black peer block h-12 w-full rounded-md bg-white text-sm focus:appearance-none focus:outline-none" 
                    autofocus 
                    autocomplete="new-password" 
                    aria-invalid="<#if messagesPerField.existsError('password','password-confirm')>true</#if>" 
                />
                <span class="text-brand-gray-600 pointer-events-none absolute left-5 top-3.5 origin-[0] -translate-y-3 scale-75 transform cursor-text text-sm duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:-translate-y-4 peer-focus:scale-[0.65]">
                    ${placeholder}
                </span>
                <button 
                    class="absolute right-4 top-4"
                    type="button" 
                    aria-label="show" 
                    aria-controls="${id}"
                    data-password-toggle 
                    data-label-show="show" 
                    data-label-hide="hide"
                >
                    <i class="fa fa-eye" aria-hidden="true"></i>
                </button>
            </label>
        </div>
    </div>
</#macro>

<#macro textInput id placeholder errorMessageKey>
    <div class="relative w-full rounded-md">
        <div class="flex flex-row gap-1">
            <label class="relative contents font-normal" aria-label="${id}">
                <input
                    type="text"
                    id="${id}"
                    name="${id}"
                    placeholder=""
                    class="pl-5 pt-2 border border-brand-gray-200 placeholder-brand-gray-600 focus:ring-brand-gray-500 focus:border-brand-gray-500 text-brand-black peer block h-12 w-full rounded-md bg-white text-sm focus:appearance-none focus:outline-none"
                    autofocus
                    autocomplete="new-password"
                    aria-invalid="<#if messagesPerField.existsError(errorMessageKey)>true</#if>"
                />
                <span class="text-brand-gray-600 pointer-events-none absolute left-5 top-3.5 origin-[0] -translate-y-3 scale-75 transform cursor-text text-sm duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:-translate-y-4 peer-focus:scale-[0.65]">
                    ${placeholder}
                </span>
            </label>
        </div>
    </div>
</#macro>
