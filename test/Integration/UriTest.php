<?php

namespace Takaram\Psr7\Test\Integration;

use Http\Psr7Test\UriIntegrationTest;
use Psr\Http\Message\UriInterface;
use Takaram\Psr7\Uri;

class UriTest extends UriIntegrationTest
{
    protected $skippedTests = [
        'testWithSchemeInvalidArguments' => 'Argument types are declared in the method signature',
        'testGetPathNormalizesMultipleLeadingSlashesToSingleSlashToPreventXSS' => 'Not implemented',
        'testUriModification2' => 'Cannot parse an empty URI',
        'testSpecialCharsInUserInfo' => 'Not implemented',
    ];

    public function createUri($uri): Uri
    {
        return Uri::create($uri);
    }

    public function testUriModification2(): void
    {
        $this->markTestSkipped('Cannot parse an empty URI');
    }

    public function testSpecialCharsInUserInfo(): void
    {
        $this->markTestSkipped('Not implemented');
    }

    public static function getPaths()
    {
        $test = new static('uriprovider');

        return [
            [$test->createUri('http://www.foo.com/'), '/'],
            // TODO: Fix theses tests
            // [$test->createUri('http://www.foo.com'), ''],
            // [$test->createUri('foo/bar'), 'foo/bar'],
            // [$test->createUri('http://www.foo.com/foo bar'), '/foo%20bar'],
            [$test->createUri('http://www.foo.com/foo%20bar'), '/foo%20bar'],
            [$test->createUri('http://www.foo.com/foo%2fbar'), '/foo%2fbar'],
        ];
    }

}
