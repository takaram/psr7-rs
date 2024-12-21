<?php
declare(strict_types=1);

namespace Takaram\Psr7;

use Psr\Http\Message\UriInterface;

class Uri implements UriInterface
{

    public static function create(string $uri = ''): self
    {
        return new self(new Internal\Uri($uri));
    }

    private function __construct(private Internal\Uri $internal)
    {
    }

    public function getScheme(): string
    {
        return $this->internal->getScheme();
    }

    public function getAuthority(): string
    {
        return $this->internal->getAuthority();
    }

    public function getUserInfo(): string
    {
        return $this->internal->getUserInfo();
    }

    public function getHost(): string
    {
        return $this->internal->getHost();
    }

    public function getPort(): ?int
    {
        return $this->internal->getPort();
    }

    public function getPath(): string
    {
        return $this->internal->getPath();
    }

    public function getQuery(): string
    {
        return $this->internal->getQuery();
    }

    public function getFragment(): string
    {
        return $this->internal->getFragment();
    }

    public function withScheme(string $scheme): UriInterface
    {
        return new self($this->internal->withScheme($scheme));
    }

    public function withUserInfo(string $user, ?string $password = null): UriInterface
    {
        return new self($this->internal->withUserInfo($user, $password));
    }

    public function withHost(string $host): UriInterface
    {
        return new self($this->internal->withHost($host));
    }

    public function withPort(?int $port): UriInterface
    {
        return new self($this->internal->withPort($port));
    }

    public function withPath(string $path): UriInterface
    {
        return new self($this->internal->withPath($path));
    }

    public function withQuery(string $query): UriInterface
    {
        return new self($this->internal->withQuery($query));
    }

    public function withFragment(string $fragment): UriInterface
    {
        return new self($this->internal->withFragment($fragment));
    }

    public function __toString(): string
    {
        return $this->internal->__toString();
    }

}
