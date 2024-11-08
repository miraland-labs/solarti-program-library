# Security Policy

1. [Reporting security problems](#reporting)
1. [Security Bug Bounties](#bounty)
1. [Scope](#scope)
1. [Incident Response Process](#process)

<a name="reporting"></a>

## Reporting security problems in the Solarti Program Library

**DO NOT CREATE A GITHUB ISSUE** to report a security problem.

Instead please use this [Report a Vulnerability](https://github.com/miraland-labs/miraland-program-library/security/advisories/new) link.
Provide a helpful title and detailed description of the problem.

If you haven't done so already, please **enable two-factor auth** in your GitHub account.

Expect a response as fast as possible in the advisory, typically within 72 hours.

--

If you do not receive a response in the advisory, send an email to
security@miraland.io with the full URL of the advisory you have created. DO NOT
include attachments or provide detail sufficient for exploitation regarding the
security issue in this email. **Only provide such details in the advisory**.

If you do not receive a response from security@miraland.io please followup with
the team directly. You can do this in the `#core-technology` channel of the
[Miraland Tech discord server](https://miraland.io/discord), by pinging the admins
in the channel and referencing the fact that you submitted a security problem.

<a name="bounty"></a>

## Security Bug Bounties

The Miraland Foundation offer bounties for critical Miraland security issues. Please
see the [Miraland Security Bug
Bounties](https://github.com/miraland-labs/miraland/security/policy#security-bug-bounties)
for details on classes of bugs and payment amounts.

<a name="scope"></a>

## Scope

Only a subset of programs within the Solarti Program Library repo are deployed to
the Miraland Mainnet Beta. Currently, this includes:

-   [associated-token-account](https://github.com/miraland-labs/miraland-program-library/tree/master/associated-token-account/program)
-   [feature-proposal](https://github.com/miraland-labs/miraland-program-library/tree/master/feature-proposal/program)
-   [governance](https://github.com/miraland-labs/miraland-program-library/tree/master/governance/program)
-   [memo](https://github.com/miraland-labs/miraland-program-library/tree/master/memo/program)
-   [name-service](https://github.com/miraland-labs/miraland-program-library/tree/master/name-service/program)
-   [stake-pool](https://github.com/miraland-labs/miraland-program-library/tree/master/stake-pool/program)
-   [token](https://github.com/miraland-labs/miraland-program-library/tree/master/token/program)

If you discover a critical security issue in an out-of-scope program, your finding
may still be valuable.

Many programs, including
[token-swap](https://github.com/miraland-labs/miraland-program-library/tree/master/token-swap/program)
and [token-lending](https://github.com/miraland-labs/miraland-program-library/tree/master/token-lending/program),
have been forked and deployed by prominent ecosystem projects, many of which
have their own bug bounty programs.

While we cannot guarantee a bounty from another entity, we can help determine who
may be affected and put you in touch the corresponding teams.

<a name="process"></a>

## Incident Response Process

In case an incident is discovered or reported, the
[Miraland Security Incident Response Process](https://github.com/miraland-labs/miraland/security/policy#incident-response-process)
will be followed to contain, respond and remediate.
