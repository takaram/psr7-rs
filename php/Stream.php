<?php
declare(strict_types=1);

namespace Takaram\Psr7;

use Psr\Http\Message\StreamInterface;
use RuntimeException;

class Stream implements StreamInterface
{

    /**
     * @param resource $resource
     */
    public static function createFromResource($resource): self
    {
        $body = stream_get_contents($resource);
        if ($body === false) {
            throw new RuntimeException('Unable to read from stream');
        }
        return new self(new Internal\VecStream($body));
    }

    private function __construct(private Internal\VecStream $internal)
    {
    }

    public function __toString(): string
    {
        return $this->internal->__toString();
    }

    public function close(): void
    {
        $this->internal->close();
    }

    public function detach()
    {
        return $this->internal->detach();
    }

    public function getSize(): ?int
    {
        return $this->internal->getSize();
    }

    public function tell(): int
    {
        return $this->internal->tell();
    }

    public function eof(): bool
    {
        return $this->internal->eof();
    }

    public function isSeekable(): bool
    {
        return $this->internal->isSeekable();
    }

    public function seek(int $offset, int $whence = SEEK_SET): void
    {
        $this->internal->seek($offset, $whence);
    }

    public function rewind(): void
    {
        $this->internal->rewind();
    }

    public function isWritable(): bool
    {
        return $this->internal->isWritable();
    }

    public function write(string $string): int
    {
        return $this->internal->write($string);
    }

    public function isReadable(): bool
    {
        return $this->internal->isReadable();
    }

    public function read(int $length): string
    {
        return $this->internal->read($length);
    }

    public function getContents(): string
    {
        return $this->internal->getContents();
    }

    public function getMetadata(?string $key = null)
    {
        return $this->internal->getMetadata($key);
    }

}
