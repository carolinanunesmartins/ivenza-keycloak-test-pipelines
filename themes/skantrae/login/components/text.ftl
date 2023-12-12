<#macro p>
    <p class="text-sm md:text-base">
        <#nested />
    </p>
</#macro>

<#macro link href>
    <a href="${href}" class="group text-sm md:text-base text-brand-secondary-500 hover:text-brand-secondary-600">
        <#nested />
    </a>
</#macro>

<#macro rightArrowLink href>
    <@link href=href>
        <span class="flex flex-row gap-2">
            <span class="group-hover:underline">
                <#nested />
            </span>
            <span class="material-symbols-outlined">arrow_right_alt</span>
        </span>
    </@link>
</#macro>
